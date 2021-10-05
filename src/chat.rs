use crate::rpc::dcs::hook::ChatMessage;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Chat {
    stream: broadcast::Sender<ChatMessage>,
}

impl Chat {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(128);
        Self { stream: tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChatMessage> {
        self.stream.subscribe()
    }

    pub fn handle_message(&self, player_id: u32, message: String, all: bool) {
        // if there are no active chat streams, ignore the message
        if self.stream.receiver_count() == 0 {
            return;
        }

        self.stream
            .send(ChatMessage {
                player_id,
                message: message.clone(),
                all,
            })
            .ok();
    }
}
