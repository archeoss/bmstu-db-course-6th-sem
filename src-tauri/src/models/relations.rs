use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Relations {
    Mentions {
        #[serde(skip)]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        out: Entity,
    },
    Asks {
        #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        out: Entity,
    },
    Gives {
        #[serde(skip)]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        out: Entity,
    },
    Includes {
        #[serde(skip)]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        out: Entity,
    },
    ParticipateIn {
        #[serde(skip)]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        out: Entity,
    },
    Follows {
        #[serde(skip)]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        order: usize,
        out: Entity,
    },
    AnsweredBy {
        #[serde(skip)]
        id: Uuid,
        #[serde(rename = "in")]
        into: Entity,
        order: usize,
        out: Entity,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Entity {
    Answer(Uuid),
    Computer(Uuid),
    Human(Uuid),
    Interrogator(Uuid),
    Question(Uuid),
    Session(Uuid),
    Verdict(Uuid),
}

pub struct Mentions;
pub struct Asks;
pub struct Gives;
pub struct Includes;
pub struct ParticipateIn;
pub struct Follows;
pub struct AnsweredBy;
// pub struct Human {
//     #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
//     pub id: Uuid,
//     pub name: String,
//     pub age: u8,
//     pub gender: String,
//     pub nationality: String,
// }
//
// #[allow(clippy::var)]
impl HasId for Relations {
    fn id(&mut self) -> &mut Uuid {
        match self {
            Self::Follows {
                id,
                into: _,
                order: _,
                out: _,
            }
            | Self::AnsweredBy {
                id,
                into: _,
                order: _,
                out: _,
            }
            | Self::Mentions {
                id,
                into: _,
                out: _,
            }
            | Self::Asks {
                id,
                into: _,
                out: _,
            }
            | Self::Gives {
                id,
                into: _,
                out: _,
            }
            | Self::Includes {
                id,
                into: _,
                out: _,
            }
            | Self::ParticipateIn {
                id,
                into: _,
                out: _,
            } => id,
        }
    }
}
