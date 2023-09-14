use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

use super::HasRole;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Human {
    #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub nationality: String,
}

impl HasId for Human {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}

impl HasRole for Human {
    fn role() -> super::Role {
        super::Role::Human
    }
}
