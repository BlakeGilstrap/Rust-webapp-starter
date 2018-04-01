use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use handler::index::State;
use utils::schema::article;
use std::time::SystemTime;
use model::article::Article;
use model::db::DbExecutor;
use model::response::ArticleListMsgs;

pub struct ArticleList;
impl Message for ArticleList {
    type Result = Result<ArticleListMsgs, Error>;
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
    type Result = Result<ArticleListMsgs, Error>;
    fn handle(&mut self, article_list: ArticleList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;
        let mut article_result: Vec<Article> = vec![];
        article_result = article.load::<Article>(&self.0).expect("Error");
        // let conn = self.0.get().unwrap();
        // for row in conn.execute("SELECT article.id, article.user_id, article.category, article.title, article.body, article.created_at 
        //                    FROM article ", &[]).unwrap(){
        //         let mut result = Article {
        //         id: row.get(0),
        //         user_id: row.get(1),
        //         category: row.get(2),
        //         title: row.get(3),
        //         body: row.get(4),
        //         created_at: row.get(5),
        //     };
        //     article_result.push(result);
        
        // }
        Ok(ArticleListMsgs { 
                status: 200,
                message : "article_list result.".to_string(),
                article_list: article_result,
        })
    }
}