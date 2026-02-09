use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool")
}
