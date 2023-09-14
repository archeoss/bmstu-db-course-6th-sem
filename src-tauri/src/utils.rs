use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use uuid::Uuid;

pub const CREDENTIALS_FILE: &str = "./build/credentials.json";

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub ns: String,
    pub db: String,
    pub sc: String,
}

pub fn read_root_credential(filename: &str) -> Result<Credentials> {
    let credential: Credentials = serde_json::from_str(&read_file(filename)?)?;

    Ok(credential)
}

pub fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub trait HasId {
    fn id(&mut self) -> &mut Uuid;
}

pub fn parse_thing_uuid(id: &str) -> Result<Uuid> {
    let mut id = dbg!(id.to_string());
    if id.contains('⟨') {
        id.pop();
        if !id.is_empty() {
            id.remove(0);
        }
    }
    Ok(Uuid::parse_str(&id)?)
}

pub async fn connect_root_surreal() -> Result<Surreal<Client>> {
    let credentials = crate::utils::read_root_credential(crate::utils::CREDENTIALS_FILE)?;
    let root = Root {
        username: &credentials.username,
        password: &credentials.password,
    };

    let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port)).await?;
    db.signin(root).await?;
    db.use_ns(credentials.ns).use_db(credentials.db).await?;

    Ok(db)
}
