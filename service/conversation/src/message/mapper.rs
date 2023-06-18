use crate::conversation::pb::Message as MessageMessage;
use crate::message::entity::Message;

impl From<MessageMessage> for Message {
    fn from(value: MessageMessage) -> Self {
        Self {
            id: value.id,
            chat_id: value.chat_id,
            sender_id: value.sender_id,
            text: if value.text.is_empty() { None } else { Some(value.text) },
            images: value.images,
            is_delivered: value.is_delivered,
            is_read: value.is_read,
            sent_at: value.sent_at,
        }
    }
}

impl From<Message> for MessageMessage {
    fn from(value: Message) -> Self {
        Self {
            id: value.id,
            chat_id: value.chat_id,
            sender_id: value.sender_id,
            text: if let Some(text) = value.text { text } else { String::new() },
            images: value.images,
            is_delivered: false,
            is_read: false,
            sent_at: value.sent_at,
        }
    }
}