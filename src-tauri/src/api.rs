use std::error::Error;

use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::{Jwt, Scope},
    Surreal,
};

use crate::{
    models::Role,
    repository::surrealdb::{AuthParams, LoginParams},
    utils::Credentials,
};

// use crate::utils::Credentials;
//
// fn new_db(
//     Credentials {
//         username,
//         password,
//         host,
//         port,
//         ns,
//         db,
//         sc,
//     }: Credentials,
//     role: Role,
// ) {
// }
//
//

use crate::errors::api::Err;

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

    // todo!()
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
    let connection = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;
    let res = connection
        .signup(Scope {
            namespace: &ns,
            database: &db,
            scope: &sc,
            params: AuthParams {
                user: &username,
                password: &password,
                role: role.into(),
            },
        })
        .await?;
    Ok(res)

    // todo!()
}
