use crate::models::*;
use rocket::{get, serde::json::Json};

#[get("/todos")]
pub fn get_todos() -> Json<Vec<Todo>> {
    Json(vec![
        Todo {
            text: "123".to_string(),
            done: true,
        },
        Todo {
            text: "456".to_string(),
            done: false,
        },
    ])
}
