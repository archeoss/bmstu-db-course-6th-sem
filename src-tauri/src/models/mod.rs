use serde::{Deserialize, Serialize};

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
