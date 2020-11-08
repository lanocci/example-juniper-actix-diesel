use super::config;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod model;
pub mod repository;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(config::DATABASE_URL());
    Pool::new(manager).expect("Failed to create db connection pool.")
}
