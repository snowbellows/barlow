use super::database;
use super::database::{PgPool, PooledPg};
use super::models::*;
use warp::{http::StatusCode, Filter};

pub fn routes(database_pool: PgPool) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
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

    let blog_id = blog.and(warp::path::param::<u16>());

    // GET /api/v1/blog
    let blog_list = warp::get2()
        .and(blog_index)
        .and(pg.clone())
        .and_then(list_posts);

    // POST /api/v1/blog
    let blog_create = warp::post2()
        .and(blog_index)
        .and(json_body)
        .and(pg.clone())
        .and_then(create_post);

    // PUT /api/v1/blog/:id
    let blog_update = warp::put2()
        .and(blog_id)
        .and(json_body)
        .and(pg.clone())
        .and_then(update_post);

    // PUT /api/v1/blog/:id/publish
    let blog_publish = warp::put2()
        .and(blog_id)
        .and(warp::path("publish"))
        .and(pg.clone())
        .and_then(publish_post);

    let api = blog_list.or(blog_create).or(blog_update).or(blog_publish);

    let index = warp::fs::file("static/index.html").and(warp::path::end());

    api.or(index).with(warp::log("barlow")).boxed()
}

// API Handlers

/// GET /api/v1/blog
fn list_posts(conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    database::load_posts_5_published(conn)
        .map(|ref posts| warp::reply::json(posts))
        .map_err(|e| e.into())
}

/// POST /api/v1/blog
fn create_post(new: NewPost, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    database::insert_post(new, conn)
        .map(|ref post| warp::reply::with_status(warp::reply::json(post), StatusCode::CREATED))
        .map_err(|e| e.into())
}

/// PUT /api/v1/blog/:id
fn update_post(
    id: u16,
    update: NewPost,
    conn: PooledPg,
) -> Result<impl warp::Reply, warp::Rejection> {
    database::update_post(id.into(), update, conn)
        .map(|ref post| warp::reply::json(post))
        .map_err(|e| e.into())
}

/// PUT /api/v1/blog/:id/publish
fn publish_post(id: u16, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    database::publish_post(id.into(), conn)
        .map(|ref post| warp::reply::json(post))
        .map_err(|e| e.into())
}
