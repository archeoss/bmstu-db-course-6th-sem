use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

trait Repository<T> {
    async fn new() -> Self;
    async fn get(&self, id: &str) -> Option<T>;
    async fn put(&self, id: &str, value: T) -> Option<T>;
    async fn delete(&self, id: &str) -> Option<T>;
    async fn get_all(&self) -> Vec<T>;
    async fn delete_all(&self) -> Vec<T>;
}
