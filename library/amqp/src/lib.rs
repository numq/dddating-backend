use std::error::Error;

use futures_lite::stream::StreamExt;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Consumer, ExchangeKind};
use lapin::options::{BasicAckOptions, BasicCancelOptions, BasicConsumeOptions, BasicPublishOptions, BasicRejectOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions, QueuePurgeOptions};
use lapin::protocol::constants::REPLY_SUCCESS;
use lapin::types::FieldTable;
use tokio::spawn;

#[derive(Clone)]
pub struct MessageQueue {
    channel: Channel,
    consumer: Option<Consumer>,
}

impl MessageQueue {
    pub async fn connect(hostname: &str, port: &str) -> Result<Self, Box<dyn Error>> {
        let uri = &format!("amqp://{}:{}", hostname, port);
        let connection = Connection::connect(uri, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        println!("connected to message queue with uri: {}", uri);

        Ok(Self { channel, consumer: None })
    }

    pub async fn close(&self) -> Result<(), Box<dyn Error>> {
        match self.channel.close(REPLY_SUCCESS, "ok").await {
            Ok(()) => {
                println!("disconnected from message queue");
                Ok(())
            }
            Err(err) => Err(Box::new(err))
        }
    }

    pub async fn bind_queue(
        &self,
        exchange_name: &str,
        exchange_type: &str,
        queue_name: &str,
        routing_key: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.channel
            .exchange_declare(
                exchange_name,
                ExchangeKind::Custom(String::from(exchange_type)),
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        self.channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        self.channel
            .queue_bind(
                queue_name,
                exchange_name,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;
        Ok(())
    }

    pub async fn clear_queue(&self, queue_name: &str) -> Result<(), Box<dyn Error>> {
        match self.channel.queue_purge(queue_name, QueuePurgeOptions::default()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err))
        }
    }

    pub async fn publish(
        &self,
        exchange_name: &str,
        routing_key: &str,
        message: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        self.channel
            .basic_publish(
                exchange_name,
                routing_key,
                BasicPublishOptions::default(),
                message,
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }

    pub async fn start_consuming(
        &mut self,
        queue_name: &str,
        mut callback: Box<dyn FnMut(&[u8]) -> Result<(), Box<dyn Error + Send>> + Send>,
    ) -> Result<(), Box<dyn Error>> {
        let consumer = self.channel
            .basic_consume(
                queue_name,
                "default",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            ).await.unwrap();
        self.consumer = Some(consumer.clone());
        let channel = self.channel.clone();

        spawn(async move {
            println!("consumer with name '{}' started", &consumer.tag().as_str());

            while let Some(delivery) = consumer.clone().next().await {
                if let Ok(delivery) = delivery {
                    if let Ok(()) = callback(&delivery.data) {
                        channel
                            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                            .await
                            .unwrap();
                    } else {
                        channel
                            .basic_reject(delivery.delivery_tag, BasicRejectOptions::default())
                            .await
                            .unwrap();
                    }
                }
            }
        });
        Ok(())
    }

    pub async fn stop_consuming(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(consumer) = self.consumer.take() {
            self.channel.basic_cancel(consumer.tag().as_str(), BasicCancelOptions::default()).await?;
            self.consumer = None;
            print!("consumer stopped");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, SystemTime};

    use tokio::time::sleep;

    use super::*;

    const HOSTNAME: &str = "127.0.0.1";
    const PORT: &str = "5672";

    const EXCHANGE_NAME: &str = "test-exchange";
    const QUEUE_NAME: &str = "test-queue";
    const ROUTING_KEY: &str = "test-routing-key";
    const TEST_MESSAGE: &str = "test";

    #[tokio::test]
    async fn test_connect_disconnect() {
        let queue = match MessageQueue::connect(HOSTNAME, PORT).await {
            Ok(queue) => queue,
            Err(err) => panic!("failed to connect to message queue: {}", err),
        };
        match queue.close().await {
            Ok(_) => println!("disconnected from message queue"),
            Err(err) => panic!("failed to disconnect from message queue: {}", err),
        }
    }

    #[tokio::test]
    async fn test_bind_clear() {
        let message_queue = MessageQueue::connect(HOSTNAME, PORT).await.unwrap();

        assert_eq!(message_queue.bind_queue(EXCHANGE_NAME, "direct", QUEUE_NAME, ROUTING_KEY).await.unwrap(), ());
        println!("queue bounded successfully");

        assert_eq!(message_queue.clear_queue(QUEUE_NAME).await.unwrap(), ());
        println!("queue cleared successfully");
    }

    #[tokio::test]
    async fn test_publish_consume() {
        let mut message_queue = MessageQueue::connect(HOSTNAME, PORT).await.unwrap();

        let expected_message_count = 5;

        let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let callback_messages = Arc::clone(&messages);

        for _ in 0..expected_message_count {
            assert_eq!(message_queue.publish(EXCHANGE_NAME, ROUTING_KEY, TEST_MESSAGE.as_bytes()).await.unwrap(), ());
        }

        let handle_message = move |msg: &[u8]| -> Result<(), Box<dyn Error + Send>> {
            let received_message = from_utf8(msg).unwrap();
            let expected_message = TEST_MESSAGE;
            assert_eq!(received_message, expected_message);
            let mut vec = callback_messages.lock().unwrap();
            vec.push(String::from(received_message));
            Ok(())
        };

        let mut consuming_queue = message_queue.clone();
        consuming_queue.start_consuming(QUEUE_NAME, Box::new(handle_message)).await.unwrap();

        let now = SystemTime::now();
        while messages.lock().unwrap().len() < expected_message_count {
            if now.elapsed().unwrap().as_millis() > 5000 {
                panic!("a timeout occured")
            }
            sleep(Duration::from_millis(100)).await;
        }

        assert_eq!(messages.lock().unwrap().len(), expected_message_count);

        assert_eq!(message_queue.stop_consuming().await.unwrap(), ());
    }
}
