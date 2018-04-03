use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use handler::index::State;
use utils::schema::article;
use std::time::SystemTime;
use model::article::{ Article, ArticleId, NewArticle, ArticleNew };
use model::db::DbExecutor;
use model::response::{ ArticleListMsgs, ArticleMsgs, Msgs };

impl Message for ArticleId {
    type Result = Result<ArticleMsgs, Error>;
}
pub fn article(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
        let dbexecutor = req.state().db.clone();
        let header_article_id = req.match_info().get("article_id").unwrap();
        let article_id: i32 = header_article_id.parse().unwrap();
        dbexecutor.send(ArticleId{
                   article_id: article_id,
                   })         
                   .from_err()
                    .and_then(|res| {
                            match res {
                                Ok(article) => Ok(httpcodes::HTTPOk.build().json(article)?),
                                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                            }
                    }).responder()                   

}

impl Handler<ArticleId> for DbExecutor {
    type Result = Result<ArticleMsgs, Error>;
    fn handle(&mut self, article_id: ArticleId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::article::dsl::*;
        let article_result =  article.filter(&id.eq(&article_id.article_id)).load::<Article>(&self.0);
        let the_article = match article_result {
            Ok(ref some_article) => match some_article.first() {
                Some(a_article) => Some(a_article.clone()),
                None => None,
            },
            Err(_) => None,
        };
        match the_article {
            Some(the_article) => {
                    let current_article = Article {
                            id: the_article.id,
                            user_id: the_article.user_id,
                            category: the_article.category.clone(),
                            title: the_article.title.clone(),
                            body: the_article.body.clone(),
                            created_at: the_article.created_at.clone(),
                    };
                    Ok(ArticleMsgs { 
                            status: 200,
                            message : "The  current_user info.".to_string(),
                            article: current_article,
                    })
            },
            None => {
                    let no_article = Article {
                            id: 0,
                            user_id: 0,
                            category: "".to_owned(),
                            title: "".to_owned(),
                            body: "".to_owned(),
                            created_at: SystemTime::now(),
                    };
                    Ok(ArticleMsgs { 
                            status: 400,
                            message : "error.".to_string(),
                            article: no_article,
                    })
            },
        }
    }
}








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
                user_id: article_new.user_id,
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
                user_id: article_new.user_id,
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