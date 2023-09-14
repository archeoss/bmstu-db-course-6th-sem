use crate::models::{HasRole, User};
use crate::prelude::*;
use ::surrealdb::opt::auth::Jwt;
use std::error::Error;
use uuid::Uuid;

pub mod surrealdb;

pub trait CrudOps<T> {
    async fn get(&self, id: Uuid) -> Result<T, Box<dyn Error>>;
    async fn put(&self, id: Uuid, value: T) -> Result<Option<T>, Box<dyn Error>>;
    async fn delete(&self, id: Uuid) -> Result<T, Box<dyn Error>>;
    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
    async fn delete_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
}

pub trait MetaOps {
    async fn get_meta(&self) -> Result<User, Box<dyn Error>>;
}

pub trait Repository<T, C>
where
    C: CrudOps<T>,
{
    fn new(connection: C) -> Result<Self>
    where
        Self: std::marker::Sized;
    fn connection(&self) -> C;
    async fn get<G>(&self, id: Uuid) -> Result<T, Box<dyn Error>> {
        self.connection().get(id).await
    }
    async fn put(&self, id: Uuid, value: T) -> Result<Option<T>, Box<dyn Error>> {
        self.connection().put(id, value).await
    }

    async fn delete(&self, id: Uuid) -> Result<T, Box<dyn Error>> {
        self.connection().delete(id).await
    }

    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        self.connection().get_all().await
    }

    async fn delete_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        self.connection().delete_all().await
    }
}

pub trait Utils<T, C>
where
    Self: Repository<T, C>,
    C: MetaOps + CrudOps<T>,
    T: HasRole,
{
    async fn meta_from_jwt(&self, token: Jwt) -> Result<User, Box<dyn Error>> {
        self.connection().get_meta().await
    }
}
