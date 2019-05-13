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

        use super::schema::posts::dsl::{body, posts, title};

        diesel::update(posts.find(id))
                .set((title.eq(update.title), body.eq(update.body)))
                .get_result(&conn)
                .map_err(|e| ServerError::Database(e))
}

/// Publish a post
pub fn publish_post(id: i32, conn: PooledPg) -> Result<Post> {
        debug!("Publish blog post {}", id);

        use super::schema::posts::dsl::{posts, published};

        diesel::update(posts.find(id))
                .set(published.eq(true))
                .get_result(&conn)
                .map_err(|e| ServerError::Database(e))
}

#[cfg(test)]
mod tests {
        use super::*;
        use crate::test_utils::test_connection;
        use proptest::prelude::*;

        #[test]
        fn it_publishes_blog_post() {
                let conn = test_connection();

                let returned_post = publish_post(1, conn).expect("publish_post should not fail");

                assert_eq!(returned_post.published, true)
        }

        proptest! {
        #[test]
        fn it_inserts_blog_post(s1 in "\\w+", s2 in "\\w+") {
                let new = NewPost {
                        title: s1.clone(),
                        body: s2.clone(),
                };

                let conn = test_connection();

                let returned_post = insert_post(new, conn).expect("insert_user should not fail");

                let correct_post = Post {
                        id: returned_post.id.clone(),
                        title: s1,
                        body: s2,
                        published: false,
                };

                assert_eq!(returned_post, correct_post)
        }

        #[test]
        fn it_updates_blog_post(s1 in "\\w+", s2 in "\\w+") {
                let new = NewPost {
                        title: s1.clone(),
                        body: s2.clone(),
                };

                let conn = test_connection();

                let returned_post = update_post(1, new, conn).expect("update_post should not fail");

                let correct_post = Post {
                        id: 1,
                        title: s1,
                        body: s2,
                        published: returned_post.published.clone(),
                };

                assert_eq!(returned_post, correct_post)
        }
        }
}
