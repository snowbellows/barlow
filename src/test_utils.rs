use super::database::PooledPg;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub fn test_connection() -> PooledPg {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::new(manager).expect("Postgres connection pool could not be created");

    let conn = pool.get().expect("Unable to get connection from pool");

    conn.begin_test_transaction().expect("Unable to begin test transaction");
    conn
}
