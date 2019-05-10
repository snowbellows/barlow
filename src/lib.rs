mod authentication;
mod database;
mod models;
mod result;
mod routes;
mod schema;
#[cfg(test)]
mod test_utils;

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
extern crate proptest;
extern crate scrypt;

use self::database::establish_pool;
use self::routes::routes;

pub fn run(database_url: String) {
    let database_pool = establish_pool(database_url);

    warp::serve(routes(database_pool)).run(([127, 0, 0, 1], 3030));
}
