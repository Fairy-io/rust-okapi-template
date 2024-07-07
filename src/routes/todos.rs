use crate::{
    db::connection::ConnectionPool, models::*, repositories::todo_repository::repository_get_todos,
};
use rocket::{get, serde::json::Json, State};
use std::sync::Arc;

#[get("/todos")]
pub async fn get_todos(db_pool: &State<Arc<ConnectionPool>>) -> Json<Vec<Todo>> {
    let todos = repository_get_todos(db_pool).await;

    Json(todos)
}
