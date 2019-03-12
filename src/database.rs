use super::models::*;
use super::result::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_pool(database_url: String) -> PgPool {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        Pool::new(manager).expect("Postgres connection pool could not be created")
}

/// Return first 5 published blog posts
pub fn load_posts_5_published(connection: PooledPg) -> Result<Vec<Post>> {
        debug!("Load first 5 published blog posts");

        use super::schema::posts;

        posts::table
                .filter(posts::published.eq(true))
                .limit(5)
                .load::<Post>(&connection)
                .map_err(|e| ServerError::Database(e))
}

/// Create new post
pub fn insert_post(create: NewPost, conn: PooledPg) -> Result<Post> {
        debug!("Create blog post {:?}", &create);

        use super::schema::posts;

        diesel::insert_into(posts::table)
                .values(&create)
                .get_result(&conn)
                .map_err(|e| ServerError::Database(e))
}

/// Update a post
pub fn update_post(id: i32, update: NewPost, conn: PooledPg) -> Result<Post> {
        debug!("Update blog post {}: {:?}", id, update);

        use super::schema::posts::dsl::{posts, title, body};

        diesel::update(posts.find(id))
                .set((title.eq(update.title), body.eq(update.body)))
                .get_result(&conn)
                .map_err(|e| ServerError::Database(e))
                

}