use dotenv::dotenv;
use std::env;
use barlow_spa::run;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=barlow=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "barlow=info");
    }

    pretty_env_logger::init();

    run(database_url)
}
