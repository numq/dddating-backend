use std::error::Error;

use amiquip::{Channel, Connection, ConsumerMessage, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish, QueueDeclareOptions};
use serde::{de::DeserializeOwned, Serialize};

pub struct MessageQueue {
    url: String,
}

impl MessageQueue {
    fn get_connection(&self) -> Result<Connection, Box<dyn Error>> {
        Connection::insecure_open(&format!("amqp://{}", self.url)).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn new(url: &str) -> Self {
        Self { url: String::from(url) }
    }

    pub fn bind_queue(
        &self,
        exchange_name: &str,
        queue_name: &str,
        routing_key: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = self.get_connection()?;
        let channel: Channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(ExchangeType::Direct, exchange_name, ExchangeDeclareOptions::default())?;
        let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;
        let _ = queue.bind(&exchange, routing_key, FieldTable::default())?;
        Ok(())
    }

    pub fn publish<T: Serialize>(
        &self,
        exchange_name: &str,
        routing_key: &str,
        message: T,
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = self.get_connection()?;
        let channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(ExchangeType::Direct, exchange_name, ExchangeDeclareOptions::default())?;
        let message = serde_json::to_string(&message)?;
        let _ = exchange.publish(Publish::new(message.as_bytes(), routing_key))?;
        Ok(())
    }

    pub fn consume<T: DeserializeOwned>(
        &self,
        queue_name: &str,
        handler: fn(T) -> bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut connection = self.get_connection()?;
        let channel = connection.open_channel(None)?;
        let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;
        let consumer = queue.consume(Default::default())?;
        for (_, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let deserialized_message: T = serde_json::from_slice(&delivery.body)?;
                    if !handler(deserialized_message) {
                        break;
                    }
                    consumer.ack(delivery)?;
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const URL: &str = "127.0.0.1:5672";

    #[test]
    fn test_bind_publish_consume() {
        let (exchange_name, queue_name, routing_key, message) = ("exchange", "queue", "key", "message");

        let queue = MessageQueue::new(URL);
        assert_eq!(queue.bind_queue(exchange_name, queue_name, routing_key).unwrap(), ());

        assert_eq!(queue.publish(exchange_name, routing_key, message).unwrap(), ());

        fn handler(msg: String) -> bool {
            println!("{}", msg);
            assert_eq!(&msg, "message");
            false
        }

        assert_eq!(queue.consume(queue_name, handler).unwrap(), ());
    }
}
