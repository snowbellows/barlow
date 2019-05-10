use super::database::{PgPool, PooledPg};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub fn create_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Postgres connection pool could not be created")
}

pub fn connection(pool: &PgPool) -> PooledPg {
    let conn = pool.get().expect("Unable to get connection from pool");

    conn.begin_test_transaction().expect("Unable to begin test transaction");
    conn
}
