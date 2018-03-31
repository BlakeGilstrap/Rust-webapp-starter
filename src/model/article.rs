use model::user::User;
use utils::schema::article;
use std::time::SystemTime;

#[derive(Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub category: String,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
}


#[derive(Debug,Serialize,Deserialize,Insertable)]
#[table_name="article"]
pub struct NewArticle<'a> {
    pub user_id: i32,
    pub category: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub created_at: SystemTime,
}