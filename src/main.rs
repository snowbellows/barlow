pub mod models;
pub mod schema;

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

use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;
use warp::{http::StatusCode, Filter};

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

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

    //Setup connection pool as Warp filter
    let pg = warp::any()
        .map(move || database_pool.clone())
        .and_then(|pool: PgPool| match pool.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(warp::reject::custom(e)),
        });

    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    let api_v1 = warp::path("api").and(warp::path("v1"));

    let blog = api_v1.and(warp::path("blog"));

    let blog_index = blog.and(warp::path::end());

    // GET /api/v1/blog
    let blog_list = warp::get2()
        .and(blog_index)
        .and(pg.clone())
        .and_then(list_posts);

    // POST /api/v1/blog
    let blog_post = warp::post2()
        .and(blog_index)
        .and(json_body)
        .and(pg.clone())
        .and_then(create_post)
        .map(|reply| warp::reply::with_status(reply, StatusCode::CREATED));

    let api = blog_list.or(blog_post);

    let index = warp::fs::file("static/index.html").and(warp::path::end());


    let routes = api.or(index).with(warp::log("barlow"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}

fn establish_pool(database_url: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Postgres connection pool could not be created")
}

// API handlers

/// GET api/v1/blog
fn list_posts(connection: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("List blog posts");

    use self::schema::posts::dsl::*;

    posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .map(|blog_posts| warp::reply::json(&blog_posts))
        .map_err(|e| warp::reject::custom(e))

}

/// POST api/v1/blog with JSON body
fn create_post(create: NewPost, connection: PooledPg) -> Result<impl warp::Reply, warp::Rejection>  {
    debug!("Create blog post {:?}", &create);

    use self::schema::posts;

    diesel::insert_into(posts::table)
        .values(&create)
        .get_result(&connection)
        .map(|post: Post| warp::reply::json(&post))
        .map_err(|e| warp::reject::custom(e))

}