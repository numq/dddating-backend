use std::str::from_utf8;
use std::sync::Arc;

use tokio::spawn;

use amqp::MessageQueue;

use crate::recommendation::interactor::RecommendationInteractor;

const EXCHANGE_NAME: &str = "action";
const QUEUE_NAME: &str = "matchmaking";
const ROUTING_KEY: &str = "dislike";

pub struct MessageQueueHandler {
    interactor: Arc<Box<dyn RecommendationInteractor + Send + Sync>>,
    message_queue: MessageQueue,
}

impl MessageQueueHandler {
    pub fn new(interactor: Arc<Box<dyn RecommendationInteractor + Send + Sync>>, message_queue: MessageQueue) -> Self {
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
                        let _ = interactor.delete_candidate(&member_ids[0], &member_ids[1]).await;
                    });
                }
            }
            Ok(())
        };

        let _ = self.message_queue.start_consuming(QUEUE_NAME, Box::new(handle_message)).await;
    }
}