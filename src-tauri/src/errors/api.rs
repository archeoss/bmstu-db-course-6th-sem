use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use surrealdb::Error as SError;

#[derive(Debug)]
pub enum Err {
    Wrap(SError),
    Connection {
        host: String,
        port: String,
        surreal_error: SError,
    },
    General(Box<dyn Error>),
}

impl From<SError> for Err {
    fn from(value: SError) -> Self {
        Self::Wrap(value)
    }
}

// we must manually implement serde::Serialize
impl serde::Serialize for Err {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
// write!(f, "The table with given name not found. Name={table}")
impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::Connection {
                host,
                port,
                surreal_error,
            } => {
                write!(f, "Can't connect to the database. Host={host}, Port={port}. Surreal error: {surreal_error}")
            }
            Err::Wrap(err) => write!(f, "{err}"),
            Err::General(err) => write!(f, "{err}"),
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
