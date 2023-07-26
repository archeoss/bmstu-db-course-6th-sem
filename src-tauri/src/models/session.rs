use crate::utils::HasId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Session {
    #[cfg_attr(feature = "surrealdb", serde(skip_serializing, skip_deserializing))]
    pub id: Uuid,
    // #[serde(with = "ts_seconds")]
    pub time_frame: DateTime<Utc>,
    // #[serde(with = "ts_seconds")]
    pub time_spent: DateTime<Utc>,
}

impl HasId for Session {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
