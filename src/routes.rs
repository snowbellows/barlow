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

    let blog_post = blog.and(warp::path::end());

    let blog_id = blog.and(warp::path::param::<u16>());

    //GET /api/v1/login
    //    let api_login = warp::get2().and(api_v1).and(pg.clone()).and(warp::header("Authorization"));

    // GET /api/v1/blog or /api/v1/blog?page
    let blog_list = warp::get2()
        .and(blog)
        .and(warp::query())
        .and(pg.clone())
        .and_then(list_posts);

    // POST /api/v1/blog
    let blog_create = warp::post2()
        .and(blog_post)
        .and(json_body)
        .and(pg.clone())
        .and_then(create_post);

    // PUT /api/v1/blog/:id
    let blog_update = warp::put2()
        .and(blog_id)
        .and(warp::path::end())
        .and(json_body)
        .and(pg.clone())
        .and_then(update_post);

    // PUT /api/v1/blog/:id/publish
    let blog_publish = warp::put2()
        .and(blog_id)
        .and(warp::path("publish"))
        .and(warp::path::end())
        .and(pg.clone())
        .and_then(publish_post);

    // DELETE /api/v1/blog/:id
    let blog_delete = warp::delete2()
        .and(blog_id)
        .and(warp::path::end())
        .and(pg.clone())
        .and_then(delete_post);

    let api = blog_list.or(blog_create).or(blog_update).or(blog_publish).or(blog_delete);

    let index = warp::fs::dir("static").and(warp::path::end());

    api.or(index).with(warp::log("barlow")).boxed()
}

// API Handlers

/// GET /api/v1/login
// fn login(auth_head: String, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {

// }

/// GET /api/v1/blog or /api/v1/blog?page
fn list_posts(page: Page, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    let page_number = if let Some(page_number) = page.page {
        page_number
    } else {
        1
    };

    database::load_posts_5_published(page_number, &conn)
        .map(|ref posts| warp::reply::json(posts))
        .map_err(|e| e.into())
}

/// POST /api/v1/blog
fn create_post(new: NewPost, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    database::insert_post(new, &conn)
        .map(|ref post| warp::reply::with_status(warp::reply::json(post), StatusCode::CREATED))
        .map_err(|e| e.into())
}

/// PUT /api/v1/blog/:id
fn update_post(
    id: u16,
    update: NewPost,
    conn: PooledPg,
) -> Result<impl warp::Reply, warp::Rejection> {
    database::update_post(id.into(), update, &conn)
        .map(|ref post| warp::reply::json(post))
        .map_err(|e| e.into())
}

/// PUT /api/v1/blog/:id/publish
fn publish_post(id: u16, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    database::publish_post(id.into(), &conn)
        .map(|ref post| warp::reply::json(post))
        .map_err(|e| e.into())
}

/// DELETE /api/v1/blog/:id
fn delete_post(id: u16, conn: PooledPg) -> Result<impl warp::Reply, warp::Rejection> {
    database::delete_post(id.into(), &conn)
    .map(|ref post| warp::reply::json(post))
    .map_err(|e| e.into())
}