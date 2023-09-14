use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::HasId;

pub mod answer;
pub mod computer;
pub mod graph;
pub mod human;
pub mod interrogator;
pub mod question;
pub mod relations;
pub mod session;
pub mod verdict;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Human,
    Computer,
    Interrogator,
}

impl From<Role> for String {
    fn from(value: Role) -> Self {
        match value {
            Role::Human => "human".into(),
            Role::Computer => "computer".into(),
            Role::Interrogator => "interrogator".into(),
        }
    }
}

pub trait HasRole: HasId {
    fn role() -> Role;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub user: String,
    pub password: String,
    pub role: Role,
}
