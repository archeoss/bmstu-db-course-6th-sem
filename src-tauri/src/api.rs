use crate::errors::api::Err;
use crate::models::User;
use crate::repository::MetaOps;
use crate::{
    models::Role,
    prelude::*,
    repository::surrealdb::{AuthParams, LoginParams},
    utils::Credentials,
};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::{Jwt, Scope},
    Surreal,
};

#[derive(Deserialize)]
pub struct Context {
    pub host: String,
    pub port: u16,
    pub ns: String,
    pub db: String,
    pub sc: String,
}

#[tauri::command]
pub async fn login(
    Credentials {
        username,
        password,
        host,
        port,
        ns,
        db,
        sc,
    }: Credentials,
) -> Result<Jwt, Err> {
    let connection = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;
    let res = connection
        .signin(Scope {
            namespace: &ns,
            database: &db,
            scope: &sc,
            params: LoginParams {
                user: &username,
                password: &password,
            },
        })
        .await?;

    Ok(res)
}

#[tauri::command]
pub async fn signup(
    Credentials {
        username,
        password,
        host,
        port,
        ns,
        db,
        sc,
    }: Credentials,
    role: Role,
) -> Result<Jwt, Err> {
    tracing::info!("received Credentials: {ns}, {db}, {username}, {host}, {port}, {role:?}");
    let connection = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;
    connection.use_ns(&ns).use_db(&db).await?;
    let sc = Scope {
        namespace: &ns,
        database: &db,
        scope: &sc,
        params: AuthParams {
            user: &username,
            password: &password,
            role: role.into(),
        },
    };

    Ok(connection.signup(sc).await?)
}

#[tauri::command]
pub async fn get_info(
    Context {
        host,
        port,
        ns,
        db,
        sc,
    }: Context,
    token: Jwt,
) -> Result<User, Err> {
    tracing::info!("received Credentials: {ns}, {db}, {host}, {port}");
    let connection = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;
    connection.use_ns(&ns).use_db(&db).await?;
    connection.authenticate(token).await?;

    connection.get_meta().await.map_err(Err::General)
}
