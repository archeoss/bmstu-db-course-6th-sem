use std::error::Error;

use uuid::Uuid;

pub mod surrealdb;
pub mod tarantool;

pub trait CrudOps<T> {
    async fn get(&self, id: Uuid) -> Result<T, Box<dyn Error>>;
    async fn put(&self, id: Uuid, value: T) -> Result<Option<T>, Box<dyn Error>>;
    async fn delete(&self, id: Uuid) -> Result<T, Box<dyn Error>>;
    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
    async fn delete_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
}

pub trait Repository<T, C>
where
    C: CrudOps<T>,
{
    fn new(connection: C) -> Result<Self, Box<dyn Error>>
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
