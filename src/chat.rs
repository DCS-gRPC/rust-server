use stubs::hook::StreamChatMessagesResponse;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Chat {
    stream: broadcast::Sender<StreamChatMessagesResponse>,
}

impl Chat {
    pub fn subscribe(&self) -> broadcast::Receiver<StreamChatMessagesResponse> {
        self.stream.subscribe()
    }

    pub fn handle_message(&self, player_id: u32, message: String, all: bool) {
        // if there are no active chat streams, ignore the message
        if self.stream.receiver_count() == 0 {
            return;
        }

        self.stream
            .send(StreamChatMessagesResponse {
                player_id,
                message,
                all,
            })
            .ok();
    }
}

impl Default for Chat {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(128);
        Self { stream: tx }
    }
}
