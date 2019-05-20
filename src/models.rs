use super::schema::posts;
use super::schema::users;
use chrono::NaiveDateTime;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Queryable, Debug, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created: NaiveDateTime,
    pub category: Option<String>,
    pub tags: Vec<String>,
}

impl Serialize for Post {
    //Manual impl as NaiveDateTime does not impl Serialize

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Post", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("published", &self.published)?;
        state.serialize_field("created", &self.created.timestamp())?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("tags", &self.tags)?;
        state.end()
    }
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Queryable, Debug, PartialEq)]
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
    pub password: String,
}

#[derive(Deserialize, Copy, Clone)]
pub struct Page {
    pub page: Option<i32>,
}
