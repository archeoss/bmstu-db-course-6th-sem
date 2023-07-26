use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Question {
    #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
    pub id: Uuid,
    pub text: String,
}

impl HasId for Question {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
