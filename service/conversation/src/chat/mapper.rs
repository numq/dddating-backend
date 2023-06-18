use crate::chat::entity::Chat;
use crate::conversation::pb::Chat as ChatMessage;

impl From<ChatMessage> for Chat {
    fn from(value: ChatMessage) -> Self {
        Self {
            id: value.id,
            member_ids: value.member_ids,
            last_message: value.last_message.map(|message| message.into()),
            typing_member_ids: value.typing_member_ids,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Chat> for ChatMessage {
    fn from(value: Chat) -> Self {
        Self {
            id: value.id,
            member_ids: value.member_ids,
            last_message: value.last_message.map(|message| message.into()),
            typing_member_ids: value.typing_member_ids,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}