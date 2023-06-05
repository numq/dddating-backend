use mongodb::bson::Bson;

use crate::support::entity::{Status, Ticket};
use crate::support::pb::{Status as StatusMessage, Ticket as TicketMessage};

impl From<Status> for Bson {
    fn from(value: Status) -> Self {
        let status = match value {
            Status::Active => "Active",
            Status::Canceled => "Canceled",
            Status::Closed => "Closed",
        };
        Bson::String(status.to_owned())
    }
}

impl From<TicketMessage> for Ticket {
    fn from(value: TicketMessage) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            topic: value.topic,
            description: value.description,
            status: match StatusMessage::from_i32(value.status).unwrap() {
                StatusMessage::Active => Status::Active,
                StatusMessage::Canceled => Status::Canceled,
                StatusMessage::Closed => Status::Closed,
            },
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Ticket> for TicketMessage {
    fn from(value: Ticket) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            topic: value.topic,
            description: value.description,
            status: value.status as i32,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}