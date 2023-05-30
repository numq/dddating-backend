use crate::account::entity::Role;
use crate::authentication::entity::TokenPayload;
use crate::authentication::pb::{Role as RoleMessage, Token as TokenMessage};

impl From<TokenMessage> for TokenPayload {
    fn from(value: TokenMessage) -> Self {
        Self {
            account_id: value.account_id,
            role: match RoleMessage::from_i32(value.role).unwrap() {
                RoleMessage::User => Role::User,
                RoleMessage::Moderator => Role::Moderator
            },
        }
    }
}

impl From<TokenPayload> for TokenMessage {
    fn from(value: TokenPayload) -> Self {
        Self {
            account_id: value.account_id,
            role: i32::from(match value.role {
                Role::User => RoleMessage::User,
                Role::Moderator => RoleMessage::Moderator
            }),
        }
    }
}