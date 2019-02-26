#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate warp;
extern crate dotenv;

use std::env;
use warp::{http::StatusCode, Filter};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=barlow=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "barlow=info");
    }
    pretty_env_logger::init();

    let index = warp::fs::file("static/index.html");

    warp::serve(index).run(([127, 0, 0, 1], 3030));
}
