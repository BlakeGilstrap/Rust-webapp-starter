use utils::schema::users;
use std::time::SystemTime;
use chrono::{DateTime,Utc,NaiveDateTime};

#[derive(Debug,Serialize,PartialEq,Identifiable,Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: SystemTime,
}

#[derive(Debug,Deserialize,Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: SystemTime,
}