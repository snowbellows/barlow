use super::schema::posts;
use super::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub created: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub password: String
}