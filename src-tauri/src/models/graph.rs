use crate::{
    repository::{
        surrealdb::{AuthParams, SurrealRepo},
        Repository,
    },
    utils::Credentials,
};

use super::{
    answer::Answer, computer::Computer, human::Human, interrogator::Interrogator,
    question::Question, relations::Relations, session::Session, verdict::Verdict, Role,
};
use std::{collections::HashMap, error::Error};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Scope, Surreal};
use uuid::Uuid;

pub struct Graph {
    pub relations: HashMap<Uuid, Relations>,
}

pub struct Database {
    answer_repo: SurrealRepo<Answer>,
    computer_repo: SurrealRepo<Computer>,
    human_repo: SurrealRepo<Human>,
    interrogator_repo: SurrealRepo<Interrogator>,
    question_repo: SurrealRepo<Question>,
    relations_repo: SurrealRepo<Relations>,
    session_repo: SurrealRepo<Session>,
    verdict_repo: SurrealRepo<Verdict>,
}


impl Database {
    pub async fn new(
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
    ) -> Result<Self, Box<dyn Error>> {
        let sc = Scope {
            namespace: &ns,
            database: &db,
            scope: &sc,
            params: AuthParams {
                user: "test",
                password: "test",
                role: role.into(),
            },
        };
        let connection = Surreal::new::<Ws>(format!("{}:{}", host, port)).await?;
        connection.signin(sc).await?;
        connection.use_ns(ns).use_db(db).await?;

        Ok(Database {
            answer_repo: SurrealRepo::<Answer>::new(connection.clone())?,
            computer_repo: SurrealRepo::<Computer>::new(connection.clone())?,
            human_repo: SurrealRepo::<Human>::new(connection.clone())?,
            interrogator_repo: SurrealRepo::<Interrogator>::new(connection.clone())?,
            question_repo: SurrealRepo::<Question>::new(connection.clone())?,
            relations_repo: SurrealRepo::<Relations>::new(connection.clone())?,
            session_repo: SurrealRepo::<Session>::new(connection.clone())?,
            verdict_repo: SurrealRepo::<Verdict>::new(connection.clone())?,
        })
    }
}
