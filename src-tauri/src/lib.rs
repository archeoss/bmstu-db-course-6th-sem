#![feature(async_fn_in_trait, async_closure, specialization)]
// Let it be for now
#![allow(dead_code)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_lifetimes)]
#![allow(missing_docs)]

pub mod api;
mod errors;
mod macros;
mod models;
mod repository;
pub mod utils;

pub mod prelude {
    // pub use crate::errors::::Err as DErr;
    #[allow(unused_imports)]
    pub use super::macros::*;
    pub use chrono::serde::ts_seconds;
    pub use color_eyre::Result;
    pub use serde::{Deserialize, Serialize};
    pub use std::error::Error;
    pub use std::sync::Arc;
    pub use std::{cell::RefCell, rc::Rc};
    pub use tokio::sync::Mutex;
}
