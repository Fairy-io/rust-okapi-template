use crate::{db::connection::ConnectionPool, models::Todo, schema::todos};
use diesel::RunQueryDsl;
use std::sync::Arc;

pub async fn repository_get_todos(db_pool: &Arc<ConnectionPool>) -> Vec<Todo> {
    let pool = db_pool.clone();

    rocket::tokio::task::spawn_blocking(move || {
        let mut connection = pool.get().unwrap();

        let todos = todos::table.load::<Todo>(&mut connection).unwrap();

        todos
    })
    .await
    .unwrap()
}
