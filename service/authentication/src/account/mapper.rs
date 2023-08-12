use crate::account::entity::{Account, Role};
use crate::account::pb::{Account as AccountMessage, Role as RoleMessage};

impl From<AccountMessage> for Account {
    fn from(value: AccountMessage) -> Self {
        Self {
            id: value.id,
            email: value.email,
            password_hash: value.password_hash,
            password_salt: value.password_salt,
            role: match RoleMessage::from_i32(value.role).unwrap() {
                RoleMessage::User => Role::User,
                RoleMessage::Moderator => Role::Moderator
            },
            premium_expiration_date: value.premium_expiration_date,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Account> for AccountMessage {
    fn from(value: Account) -> Self {
        Self {
            id: value.id,
            email: value.email,
            password_hash: value.password_hash,
            password_salt: value.password_salt,
            role: i32::from(match value.role {
                Role::User => RoleMessage::User,
                Role::Moderator => RoleMessage::Moderator
            }),
            premium_expiration_date: value.premium_expiration_date,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}