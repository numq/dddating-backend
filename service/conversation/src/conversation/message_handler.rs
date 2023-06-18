use std::str::from_utf8;
use std::sync::Arc;

use tokio::spawn;

use amqp::MessageQueue;

use crate::conversation::interactor::ConversationInteractor;

const EXCHANGE_NAME: &str = "action";
const ROUTING_KEY: &str = "chat";
const QUEUE_NAME: &str = "create";

pub struct MessageHandler {
    interactor: Arc<Box<dyn ConversationInteractor + Send + Sync>>,
    message_queue: MessageQueue,
}

impl MessageHandler {
    pub fn new(interactor: Arc<Box<dyn ConversationInteractor + Send + Sync>>, message_queue: MessageQueue) -> Self {
        Self {
            interactor,
            message_queue,
        }
    }

    pub async fn consume_new_chats(&mut self) {
        let _ = self.message_queue.bind_queue(EXCHANGE_NAME, "direct", QUEUE_NAME, ROUTING_KEY).await;

        let interactor = Arc::clone(&self.interactor);
        let handle_message = move |msg: &[u8]| {
            let interactor = Arc::clone(&interactor);
            if let Ok(value) = from_utf8(msg) {
                let member_ids: Vec<String> = value.split(",").map(String::from).collect();
                if member_ids.len() == 2 {
                    spawn(async move {
                        let _ = interactor.create_chat(member_ids).await;
                    });
                }
            }
            Ok(())
        };

        let _ = self.message_queue.start_consuming(QUEUE_NAME, Box::new(handle_message)).await;
    }
}