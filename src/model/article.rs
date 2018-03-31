use model::user::User;
use utils::schema::article;
use std::time::SystemTime;

#[derive(Debug,Serialize,Queryable,Identifiable, Associations)]
#[belongs_to(User)]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub category: String,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
}


#[derive(Deserialize,Insertable)]
#[table_name="article"]
pub struct NewArticle<'a> {
    pub user_id: i32,
    pub category: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub created_at: SystemTime,
}