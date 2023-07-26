use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    TableNotFound(String),
    GetNotFound { table: String, id: Uuid },
    DeleteNotFound { table: String, id: Uuid },
    ThingToEntityConvError { table: String, id: String },
    RelationsConvError { table: String, id: String },
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TableNotFound(table) => {
                write!(f, "The table with given name not found. Name={table}")
            }
            Self::GetNotFound { table, id } => {
                write!(
                    f,
                    "Get operation failed. Field {id} was not found in {table}."
                )
            }
            Self::DeleteNotFound { table, id } => {
                write!(
                    f,
                    "Delete operation failed. Field {id} was not found in {table}."
                )
            }
            Self::ThingToEntityConvError { table, id } => {
                write!(
                    f,
                    "Conversion from thing to entity failed. ID={id}; Table={table}."
                )
            }
            Self::RelationsConvError { table, id } => {
                write!(
                    f,
                    "Conversion from SurrealRelations to Relations failed. Unknown relation? ID={id}; Table={table}."
                )
            }
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
