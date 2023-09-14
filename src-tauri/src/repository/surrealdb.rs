use super::{CrudOps, MetaOps, Repository};
use crate::errors::surreal::Err;
use crate::models::answer::Answer;
use crate::models::computer::Computer;
use crate::models::human::Human;
use crate::models::interrogator::Interrogator;
use crate::models::question::Question;
use crate::models::relations::{Entity, Relations};
use crate::models::session::Session;
use crate::models::verdict::Verdict;
use crate::models::{Role, User};
use crate::prelude::*;
use crate::utils::{self, HasId};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Scope;
use surrealdb::sql::Id;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct SurrealCredentials {
    pub username: String,
    pub password: String,
    pub role: Role,
    pub host: String,
    pub port: u16,
    pub ns: String,
    pub db: String,
    pub sc: String,
}

#[derive(Debug, Serialize)]
pub struct SurrealUserAuth {
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Clone, Debug, Serialize)]
pub struct AuthParams<'a> {
    pub user: &'a str,
    pub password: &'a str,
    pub role: Thing,
}

#[derive(Clone, Debug, Serialize)]
pub struct LoginParams<'a> {
    pub user: &'a str,
    pub password: &'a str,
}

pub struct SurrealRepo<T> {
    connection: Surreal<Client>,
    object: PhantomData<T>,
}

async fn signin(credentials: SurrealCredentials) -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port)).await?;
    let scope = Scope {
        namespace: &credentials.ns,
        database: &credentials.db,
        scope: &credentials.sc,
        params: SurrealUserAuth {
            username: credentials.username,
            password: credentials.password,
            role: credentials.role,
        },
    };
    db.signin(scope).await?;

    Ok(db)
}

impl<T: DeserializeOwned + Serialize + Send + Sync + HasId> CrudOps<T> for Surreal<Client> {
    async fn get(&self, id: Uuid) -> Result<T, Box<dyn Error>> {
        let result: Option<T> = self
            .select((
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
                id.to_string(),
            ))
            .await?;

        result.map_or_else(
            || {
                tracing::warn!("GET: field not found");
                Err(Err::GetNotFound {
                    table: std::any::type_name::<T>()
                        .to_lowercase()
                        .rsplit("::")
                        .next()
                        .unwrap_or(&std::any::type_name::<T>().to_lowercase())
                        .to_string(),
                    id,
                }
                .into())
            },
            |mut res| {
                tracing::info!("GET: success");
                *res.id() = id;
                Ok(res)
            },
        )
    }

    async fn put(&self, id: Uuid, value: T) -> Result<Option<T>, Box<dyn Error>> {
        let result: Option<T> = self
            .update((
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
                id.to_string(),
            ))
            .content(value)
            .await?;

        Ok(result)
    }

    async fn delete(&self, id: Uuid) -> Result<T, Box<dyn Error>> {
        let result: Option<T> = self
            .delete((
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
                id.to_string(),
            ))
            .await?;

        result.map_or_else(
            || {
                tracing::warn!("DELETE: field not found");
                Err(Err::GetNotFound {
                    table: std::any::type_name::<T>()
                        .to_lowercase()
                        .rsplit("::")
                        .next()
                        .unwrap_or(&std::any::type_name::<T>().to_lowercase())
                        .to_string(),
                    id,
                }
                .into())
            },
            |res| {
                tracing::info!("DELETE: success");
                Ok(res)
            },
        )
    }

    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        Ok(self
            .select(
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
            )
            .await?)
    }

    async fn delete_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        Ok(self
            .delete(
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
            )
            .await?)
    }
}

// impl<T: DeserializeOwned> MetaOps<T> for Surreal<Client> {
impl MetaOps for Surreal<Client> {
    async fn get_meta(&self) -> Result<User, Box<dyn Error>> {
        let res: SurrealUser = self
            .select(("user"))
            .await?
            .pop()
            .ok_or(Box::from("No meta found") as Box<dyn Error>)?;

        res.try_into()
    }
}

default impl<T: DeserializeOwned + Serialize + Send + Sync + HasId> Repository<T, Surreal<Client>>
    for SurrealRepo<T>
{
    fn new(connection: Surreal<Client>) -> Result<Self> {
        Ok(Self {
            connection,
            object: PhantomData::<T>,
        })
    }

    fn connection(&self) -> Surreal<Client> {
        self.connection.clone()
    }
}

// Needed for specialization
impl Repository<Human, Surreal<Client>> for SurrealRepo<Human> {}
impl Repository<Interrogator, Surreal<Client>> for SurrealRepo<Interrogator> {}
impl Repository<Computer, Surreal<Client>> for SurrealRepo<Computer> {}
impl Repository<Answer, Surreal<Client>> for SurrealRepo<Answer> {}
impl Repository<Question, Surreal<Client>> for SurrealRepo<Question> {}
impl Repository<Session, Surreal<Client>> for SurrealRepo<Session> {}
impl Repository<Verdict, Surreal<Client>> for SurrealRepo<Verdict> {}

impl From<Entity> for Thing {
    fn from(value: Entity) -> Self {
        match value {
            Entity::Answer(id) => Self {
                tb: "answer".to_string(),
                id: Id::from(id.to_string()),
            },
            Entity::Computer(id) => Self {
                tb: "computer".to_string(),
                id: Id::from(id.to_string()),
            },
            Entity::Human(id) => Self {
                tb: "human".to_string(),
                id: Id::from(id.to_string()),
            },
            Entity::Interrogator(id) => Self {
                tb: "interrogator".to_string(),
                id: Id::from(id.to_string()),
            },
            Entity::Question(id) => Self {
                tb: "question".to_string(),
                id: Id::from(id.to_string()),
            },
            Entity::Session(id) => Self {
                tb: "session".to_string(),
                id: Id::from(id.to_string()),
            },
            Entity::Verdict(id) => Self {
                tb: "verdict".to_string(),
                id: Id::from(id.to_string()),
            },
        }
    }
}

impl TryFrom<Thing> for Entity {
    type Error = Box<dyn Error>;
    fn try_from(value: Thing) -> Result<Self, Self::Error> {
        let id = utils::parse_thing_uuid(&value.id.to_string())?;
        match value.tb.as_str() {
            "answer" => Ok(Self::Answer(id)),
            "computer" => Ok(Self::Computer(id)),
            "human" => Ok(Self::Human(id)),
            "interrogator" => Ok(Self::Interrogator(id)),
            "question" => Ok(Self::Question(id)),
            "session" => Ok(Self::Session(id)),
            "verdict" => Ok(Self::Verdict(id)),
            _ => Err(Err::ThingToEntityConvError {
                table: value.tb.clone(),
                id: value.to_string(),
            }
            .into()),
        }
    }
}

impl From<Role> for Thing {
    fn from(value: Role) -> Self {
        let id = Id::String(value.into());
        Self {
            tb: "role".to_string(),
            id,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SurrealRelations {
    pub id: Thing,
    #[serde(rename = "in")]
    pub into: Thing,
    pub out: Thing,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct SurrealUser {
    pub id: Thing,
    pub user: String,
    pub password: String,
    pub role: Thing,
}

impl TryFrom<SurrealUser> for User {
    type Error = Box<dyn Error>;

    fn try_from(value: SurrealUser) -> Result<Self, Self::Error> {
        let id = utils::parse_thing_uuid(&value.id.id.to_string())?;
        let role = match dbg!(value.role.id.to_string().as_str()) {
            "human" => Role::Human,
            "computer" => Role::Computer,
            "interrogator" => Role::Interrogator,
            _ => return Err("Unexpected error".into()),
        };

        Ok(Self {
            id,
            user: value.user,
            password: value.password,
            role,
        })
    }
}

impl TryFrom<SurrealRelations> for Relations {
    type Error = Box<dyn Error>;

    fn try_from(value: SurrealRelations) -> Result<Self, Self::Error> {
        let id = utils::parse_thing_uuid(&value.id.id.to_string())?;
        match value.id.tb.as_str() {
            "mentions" => Ok(Self::Mentions {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
            }),
            "asks" => Ok(Self::Asks {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
            }),
            "gives" => Ok(Self::Gives {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
            }),
            "includes" => Ok(Self::Includes {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
            }),
            "participateIn" => Ok(Self::ParticipateIn {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
            }),
            "follows" => Ok(Self::Follows {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
                order: value
                    .extra
                    .get("order")
                    .ok_or(Err::RelationsConvError {
                        table: value.id.tb.clone(),
                        id: value.id.id.to_string(),
                    })?
                    .as_u64()
                    .ok_or(Err::RelationsConvError {
                        table: value.id.tb.clone(),
                        id: value.id.id.to_string(),
                    })?
                    .try_into()?,
            }),
            "answeredBy" => Ok(Self::AnsweredBy {
                id,
                into: value.into.try_into()?,
                out: value.out.try_into()?,
                order: value
                    .extra
                    .get("order")
                    .ok_or(Err::RelationsConvError {
                        table: value.id.tb.clone(),
                        id: value.id.id.to_string(),
                    })?
                    .as_u64()
                    .ok_or(Err::RelationsConvError {
                        table: value.id.tb.clone(),
                        id: value.id.id.to_string(),
                    })?
                    .try_into()?,
            }),
            _ => Err(Err::RelationsConvError {
                table: value.id.tb,
                id: value.id.id.to_string(),
            }
            .into()),
        }
    }
}

impl From<Relations> for SurrealRelations {
    fn from(value: Relations) -> Self {
        match value {
            Relations::Mentions { id, into, out } => Self {
                id: Thing {
                    tb: "mentions".to_string(),
                    id: Id::from(id.to_string()),
                },
                into: into.into(),
                out: out.into(),
                extra: HashMap::default(),
            },
            Relations::Asks { id, into, out } => Self {
                id: Thing {
                    tb: "asks".to_string(),
                    id: Id::from(id.to_string()),
                },
                into: into.into(),
                out: out.into(),
                extra: HashMap::default(),
            },
            Relations::Gives { id, into, out } => Self {
                id: Thing {
                    tb: "gives".to_string(),
                    id: Id::from(id.to_string()),
                },
                into: into.into(),
                out: out.into(),
                extra: HashMap::default(),
            },
            Relations::Includes { id, into, out } => Self {
                id: Thing {
                    tb: "includes".to_string(),
                    id: Id::from(id.to_string()),
                },
                into: into.into(),
                out: out.into(),
                extra: HashMap::default(),
            },
            Relations::ParticipateIn { id, into, out } => Self {
                id: Thing {
                    tb: "participateIn".to_string(),
                    id: Id::from(id.to_string()),
                },
                into: into.into(),
                out: out.into(),
                extra: HashMap::default(),
            },
            Relations::Follows {
                id,
                into,
                order,
                out,
            } => {
                let mut extra = HashMap::new();
                extra.insert("order".to_string(), Value::Number(order.into()));
                Self {
                    id: Thing {
                        tb: "follows".to_string(),
                        id: Id::from(id.to_string()),
                    },
                    into: into.into(),
                    out: out.into(),
                    extra,
                }
            }
            Relations::AnsweredBy {
                id,
                into,
                order,
                out,
            } => {
                let mut extra = HashMap::new();
                extra.insert("order".to_string(), Value::Number(order.into()));
                Self {
                    id: Thing {
                        tb: "answeredBy".to_string(),
                        id: Id::from(id.to_string()),
                    },
                    into: into.into(),
                    out: out.into(),
                    extra,
                }
            }
        }
    }
}

impl Repository<Relations, Surreal<Client>> for SurrealRepo<Relations> {
    async fn get<G>(&self, id: Uuid) -> Result<Relations, Box<dyn Error>> {
        // self.connection.query("SELECT ")
        let rel: Option<SurrealRelations> = self
            .connection
            .select((
                std::any::type_name::<G>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<G>().to_lowercase()),
                id.to_string(),
            ))
            .await?;

        rel.ok_or(Err::GetNotFound {
            table: std::any::type_name::<G>().to_string(),
            id,
        })?
        .try_into()
    }

    async fn put(&self, id: Uuid, value: Relations) -> Result<Option<Relations>, Box<dyn Error>> {
        // self.connection().put(id, value).await
        let relation: SurrealRelations = value.into();

        let res: Option<SurrealRelations> = self
            .connection
            .update((relation.id.tb.clone(), relation.id.id.clone()))
            .content(relation)
            .await?;

        Ok(Some(
            res.ok_or(Err::GetNotFound {
                table: std::any::type_name::<Relations>().to_string(),
                id,
            })?
            .try_into()?,
        ))
    }

    async fn get_all(&self) -> Result<Vec<Relations>, Box<dyn Error>> {
        self.connection().get_all().await
    }

    async fn delete_all(&self) -> Result<Vec<Relations>, Box<dyn Error>> {
        self.connection().delete_all().await
    }
}

#[cfg(feature = "surrealdb")]
mod tests {
    #![allow(clippy::unwrap_used)]

    use surrealdb::opt::auth::Root;

    use crate::{
        models::relations::{self, Asks},
        utils::Credentials,
    };

    use super::*;

    async fn get_scope<'a>(credentials: &'a Credentials) -> Scope<'a, AuthParams<'a>> {
        Scope {
            namespace: &credentials.ns,
            database: &credentials.db,
            scope: &credentials.sc,
            params: AuthParams {
                user: "root",
                password: "toor",
                role: Role::Human.into(),
            },
        }
    }

    async fn connect() -> Surreal<Client> {
        let credentials =
            crate::utils::read_root_credential(crate::utils::CREDENTIALS_FILE).unwrap();
        let root = Root {
            username: &credentials.username,
            password: &credentials.password,
        };

        let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port))
            .await
            .unwrap();
        db.signin(root).await.unwrap();
        db.use_ns(credentials.ns)
            .use_db(credentials.db)
            .await
            .unwrap();

        db
    }

    #[tokio::test]
    async fn put_get_human() {
        let db = connect().await;
        let repo = SurrealRepo::<Human>::new(db).unwrap();
        let id = Uuid::new_v4();
        let human = Human {
            id,
            name: "test".to_string(),
            age: 10,
            gender: "Male".to_string(),
            nationality: "Nigerian".to_string(),
        };
        let _ = repo.put(id, human.clone()).await.unwrap();

        let human_get = repo.get::<Human>(id).await.unwrap();
        assert_eq!(human, human_get);
    }

    #[tokio::test]
    async fn put_get_interrogator() {
        let db = connect().await;
        let repo = SurrealRepo::<Interrogator>::new(db).unwrap();
        let id = Uuid::new_v4();
        let interrogator = Interrogator {
            id,
            age: 18,
            name: "John".into(),
            gender: "M".into(),
            nationality: "Nigerian".into(),
        };

        let _ = repo.put(id, interrogator.clone()).await.unwrap();

        let interrogator_get = repo.get::<Interrogator>(id).await.unwrap();

        assert_eq!(interrogator, interrogator_get);
    }

    #[tokio::test]
    async fn put_get_computer() {
        let db = connect().await;
        let repo = SurrealRepo::<Computer>::new(db).unwrap();
        let id = Uuid::new_v4();
        let comp = Computer {
            id,
            developed_by: "test".to_string(),
            model: "test".to_string(),
        };

        let _ = repo.put(id, comp.clone()).await.unwrap();

        let comp_get = repo.get::<Computer>(id).await.unwrap();

        assert_eq!(comp, comp_get);
    }

    #[tokio::test]
    async fn put_get_question() {
        let db = connect().await;
        let repo = SurrealRepo::<Question>::new(db).unwrap();
        let id = Uuid::new_v4();
        let quest = Question {
            id,
            text: "test".to_string(),
        };

        let _ = repo.put(id, quest.clone()).await.unwrap();

        let quest_get = repo.get::<Question>(id).await.unwrap();

        assert_eq!(quest, quest_get);
    }

    #[tokio::test]
    async fn put_get_answer() {
        let db = connect().await;
        let repo = SurrealRepo::<Answer>::new(db).unwrap();
        let id = Uuid::new_v4();
        let answer = Answer {
            id,
            text: "test".to_string(),
        };

        let _ = repo.put(id, answer.clone()).await.unwrap();

        let answer_get = repo.get::<Answer>(id).await.unwrap();

        assert_eq!(answer, answer_get);
    }

    #[tokio::test]
    async fn put_get_session() {
        let db = connect().await;
        let repo = SurrealRepo::<Session>::new(db).unwrap();
        let id = Uuid::new_v4();
        let session = Session {
            id,
            ..Default::default()
        };

        let _ = repo.put(id, session.clone()).await.unwrap();

        let session_get = repo.get::<Session>(id).await.unwrap();

        assert_eq!(session, session_get);
    }

    #[tokio::test]
    async fn put_get_verdict() {
        let db = connect().await;
        let repo = SurrealRepo::<Verdict>::new(db).unwrap();
        let id = Uuid::new_v4();
        let verdict = Verdict {
            id,
            ..Default::default()
        };

        let _ = repo.put(id, verdict.clone()).await.unwrap();

        let verdict_get = repo.get::<Verdict>(id).await.unwrap();

        assert_eq!(verdict, verdict_get);
    }

    #[tokio::test]
    async fn test_auth() {
        signup().await;
        // signin().await;
    }

    // #[tokio::test]
    async fn signup() {
        let db = connect().await;
        let credentials =
            crate::utils::read_root_credential(crate::utils::CREDENTIALS_FILE).unwrap();
        let scope = get_scope(&credentials).await; // user = test, password = test
                                                   // let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port))
                                                   //     .await
                                                   //     .unwrap();

        db.signup(scope).await.unwrap();
        let db = connect().await;
        db.query("DELETE user WHERE user = 'test'")
            .await
            .unwrap()
            .check()
            .unwrap();
    }

    // #[tokio::test]
    async fn signin() {
        let db = connect().await;
        let credentials =
            crate::utils::read_root_credential(crate::utils::CREDENTIALS_FILE).unwrap();
        let scope = get_scope(&credentials).await; // user = test, password = test
        db.signup(scope).await.unwrap();
        let scope = get_scope(&credentials).await; // user = test, password = test
        let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port))
            .await
            .unwrap();
        db.signin(scope).await.unwrap();
        let db = connect().await;
        db.query("DELETE user WHERE user = 'test'")
            .await
            .unwrap()
            .check()
            .unwrap();
    }

    #[tokio::test]
    async fn test_update_get_relations() {
        let db = connect().await;
        let repo = SurrealRepo::<Relations>::new(db).unwrap();
        let id = Uuid::new_v4();
        let relations = Relations::Asks {
            id,
            into: Entity::Question(Uuid::new_v4()),
            out: Entity::Human(Uuid::new_v4()),
        };
        let _ = repo.put(id, relations.clone()).await.unwrap();

        let relations_get = repo.get::<Asks>(id).await.unwrap();

        assert_eq!(relations, relations_get);
    }
}

// impl Serialize for Uuid {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         serializer.ser
//     }
// }
