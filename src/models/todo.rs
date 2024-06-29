use diesel::deserialize::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub done: bool,
}
