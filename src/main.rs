mod database;
mod models;
mod result;
mod routes;
mod schema;
mod authentication;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate warp;
#[macro_use]
extern crate diesel;
extern crate chrono;

use self::database::establish_pool;
use self::routes::routes;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=barlow=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "barlow=info");
    }

    pretty_env_logger::init();

    let database_pool = establish_pool(database_url);

    warp::serve(routes(database_pool)).run(([127, 0, 0, 1], 3030));
}
