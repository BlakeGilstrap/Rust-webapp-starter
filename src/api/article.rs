use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use handler::index::State;
use utils::schema::article;
use std::time::SystemTime;
use model::article::{ Article, NewArticle, ArticleNew };
use model::db::DbExecutor;
use model::response::{ ArticleListMsgs, Msgs };

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
        Ok(ArticleListMsgs { 
                status: 200,
                message : "article_list result.".to_string(),
                article_list: article_result,
        })
    }
}

pub fn article_new(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let executor = req.state().db.clone();
    req.json()                     
       .from_err()
       .and_then(move |article_new: ArticleNew| {  
            executor.send(ArticleNew{ 
                category: article_new.category,
                title: article_new.title,
                content: article_new.content,
            })         
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                    Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                }
            })
    }).responder()
}

impl Message for ArticleNew {
    type Result = Result<Msgs, Error>;
}

impl Handler<ArticleNew> for DbExecutor {
    type Result = Result<Msgs, Error>;
    fn handle(&mut self, article_new: ArticleNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;
        let new_article = NewArticle {
                user_id: 1,
                category: &article_new.category,
                title: &article_new.title,
                body: &article_new.content,
                created_at: SystemTime::now(),
        };
        diesel::insert_into(article).values(&new_article).execute(&self.0).expect("Error Article Publish");
        Ok(Msgs { 
                    status: 200,
                    message : "Article Publish Successful.".to_string(),
        })        
    }
}