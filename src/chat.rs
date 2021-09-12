use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use std::time::Duration;

use crate::rpc::dcs::hook::ChatMessage;
use tokio::sync::{broadcast, oneshot, Mutex};
use tokio::time::timeout;
use uuid::Uuid;

#[derive(Clone)]
pub struct Chat {
    stream: broadcast::Sender<ChatMessage>,
    pending: Arc<Mutex<HashMap<Uuid, oneshot::Sender<String>>>>,
}

impl Chat {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(128);
        Self {
            stream: tx,
            pending: Default::default(),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChatMessage> {
        self.stream.subscribe()
    }

    pub async fn filter_message(&self, uuid: Uuid, message: String) {
        let mut pending = self.pending.lock().await;
        if let Some(tx) = pending.remove(&uuid) {
            let _ = tx.send(message);
        }
    }

    pub async fn handle_message(&self, player_id: u32, message: String, all: bool) -> String {
        // if there are no active chat streams, return the message as is
        if self.stream.receiver_count() == 0 {
            return message;
        }

        let uuid = Uuid::new_v4();

        // add to pending to corelate the message with filter requests
        let (tx, rx) = oneshot::channel();
        let mut pending = self.pending.lock().await;
        pending.insert(uuid, tx);
        mem::drop(pending); // drop mutex lock for now

        // broadcast message to all clients
        let chat_message = ChatMessage {
            uuid: uuid.to_string(),
            player_id,
            message: message.clone(),
            all,
        };
        if self.stream.send(chat_message).is_err() {
            // all receivers are dropped, return message unfiltered
            return message;
        }

        // wait for up to 16 milliseconds for a filter request from the client
        let filtered_message = match timeout(Duration::from_millis(16), rx).await {
            // Did not receive filter request for chat message within 16ms -> drop message
            Err(_) => String::new(),
            // Channel got closed.
            Ok(Err(_)) => String::new(),
            // Received filter request for message from client.
            Ok(Ok(message)) => message,
        };

        // remove from pending
        let mut pending = self.pending.lock().await;
        pending.remove(&uuid);

        filtered_message
    }
}
