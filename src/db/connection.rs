use diesel::prelude::*;
use diesel::r2d2::*;
use std::env;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
