use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Verdict {
    #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
    pub id: Uuid,
    pub correct: bool,
}

impl HasId for Verdict {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
