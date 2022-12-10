use edgedb_derive::Queryable;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repository {
    pub id: uuid::Uuid,
    pub name: String,
    pub url: String,
    pub language: String,
}

#[derive(Queryable, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsertedRepository {
    pub id: uuid::Uuid,
}
