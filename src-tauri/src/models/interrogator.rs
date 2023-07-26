use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Interrogator {
    #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub nationality: String,
}

impl HasId for Interrogator {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
