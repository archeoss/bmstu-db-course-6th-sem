use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Computer {
    #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
    pub id: Uuid,
    pub model: String,
    pub developed_by: String,
}

impl HasId for Computer {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
