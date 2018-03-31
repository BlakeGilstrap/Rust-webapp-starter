use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use model::user::User;
use utils::schema::article;
use std::time::SystemTime;
use actix_web::{Json, Result };
use model::db::DbExecutor;

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
pub struct ArticleList;
impl Message for ArticleList {
    type Result = Result<Article, Error>;
}
pub fn article_list(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.state().db.send(ArticleList)
    .from_err()
    .and_then(|res| {
            match res {
                Ok(article_list) => Ok(httpcodes::HTTPOk.build().json(article_list)?),
                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
            }
    }).responder()
}

impl Handler<ArticleList> for DbExecutor {
    type Result = Result<Article, Error>;
    fn handle(&mut self, article_list: ArticleList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;
        let mut article_result: Vec<Article> = article::table.load(&self.0).expect("Error");
        Ok(Article { 
                id: i32,
                user_id: i32,
                category: String,
                title: String,
                body: String,
                created_at: SystemTime,
        })
    }
}