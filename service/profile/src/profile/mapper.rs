use mongodb::bson::Bson;

use crate::profile::criteria::{Basics as BasicsMessage, Filter as FilterMessage, Gender as GenderMessage};
use crate::profile::entity::{Basics, Filter, Gender, Profile};
use crate::profile::pb::Profile as ProfileMessage;

impl From<Gender> for Bson {
    fn from(value: Gender) -> Self {
        let gender = match value {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::NonBinary => "NonBinary",
        };
        Bson::String(gender.to_owned())
    }
}

impl From<BasicsMessage> for Basics {
    fn from(value: BasicsMessage) -> Self {
        Self {
            age: value.age,
            gender: match GenderMessage::from_i32(value.gender).unwrap() {
                GenderMessage::Male => Gender::Male,
                GenderMessage::Female => Gender::Female,
                GenderMessage::NonBinary => Gender::NonBinary,
            },
            location: value.location,
            preferences: value.preferences
                .iter()
                .map(|&x| match GenderMessage::from_i32(x).unwrap() {
                    GenderMessage::Male => Gender::Male,
                    GenderMessage::Female => Gender::Female,
                    GenderMessage::NonBinary => Gender::NonBinary,
                })
                .collect(),
        }
    }
}

impl From<Basics> for BasicsMessage {
    fn from(value: Basics) -> Self {
        Self {
            age: value.age,
            gender: value.gender as i32,
            location: value.location,
            preferences: value.preferences
                .iter()
                .map(|gender| i32::from(match gender {
                    Gender::Male => GenderMessage::Male,
                    Gender::Female => GenderMessage::Female,
                    Gender::NonBinary => GenderMessage::NonBinary
                }))
                .collect(),
        }
    }
}

impl From<FilterMessage> for Filter {
    fn from(value: FilterMessage) -> Self {
        Self {
            min_age: value.min_age,
            max_age: value.max_age,
            location: value.location,
            preferences: value.preferences
                .iter()
                .map(|&x| match GenderMessage::from_i32(x).unwrap() {
                    GenderMessage::Male => Gender::Male,
                    GenderMessage::Female => Gender::Female,
                    GenderMessage::NonBinary => Gender::NonBinary,
                })
                .collect(),
        }
    }
}

impl From<Filter> for FilterMessage {
    fn from(value: Filter) -> Self {
        Self {
            min_age: value.min_age,
            max_age: value.max_age,
            location: value.location,
            preferences: value.preferences
                .iter()
                .map(|gender| i32::from(match gender {
                    Gender::Male => GenderMessage::Male,
                    Gender::Female => GenderMessage::Female,
                    Gender::NonBinary => GenderMessage::NonBinary
                }))
                .collect(),
        }
    }
}

impl From<ProfileMessage> for Profile {
    fn from(value: ProfileMessage) -> Self {
        Self {
            id: value.id,
            name: value.name,
            basics: value.basics.unwrap().into(),
            bio: value.bio,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Profile> for ProfileMessage {
    fn from(value: Profile) -> Self {
        Self {
            id: value.id,
            name: value.name,
            basics: Some(value.basics.into()),
            bio: value.bio,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}