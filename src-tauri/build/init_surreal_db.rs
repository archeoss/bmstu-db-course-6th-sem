use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

const CREDENTIALS_FILE: &str = "./build/credentials.json";

#[derive(Serialize, Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub ns: String,
    pub db: String,
    pub sc: String,
}

fn read_root_credential(filename: &str) -> Result<Credentials, Box<dyn Error>> {
    let credential: Credentials = serde_json::from_str(&read_file(filename)?)?;

    Ok(credential)
}

fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub async fn create_db() -> Result<(), Box<dyn Error>> {
    let credentials = read_root_credential(CREDENTIALS_FILE)?;
    let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port)).await?;
    let scope = credentials.sc;
    let root = Root {
        username: &credentials.username,
        password: &credentials.password,
    };
    db.signin(root).await?;
    db.use_ns(credentials.ns).use_db(credentials.db).await?;

    let sql = read_file("./build/init.sql")?;
    db.query(sql).await?;

    Ok(())
}
