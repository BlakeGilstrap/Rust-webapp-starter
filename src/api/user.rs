use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use handler::index::State;
use model::db::DbExecutor;
use std::time::SystemTime;
use utils::token::verify_token;
use model::response::UserInfoMsgs;
use model::user::{ User, UserInfo };

impl Message for UserInfo {
    type Result = Result<UserInfoMsgs, Error>;
}

pub fn user_info(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
        let dbexecutor = req.state().db.clone();
        let header_token = req.headers().get("Authorization");
        
        let token = header_token.unwrap().clone();
        let token = token.to_str();
        match token {
            Ok(token) => {
                let user_token = &token[7..];
                match verify_token(user_token.to_string()) {
                    Ok(uid) => {
                        dbexecutor.send(UserInfo{
                            user_id: uid,
                        })         
                        .from_err()
                        .and_then(|res| {
                            match res {
                                Ok(user_info) => {
                                    Ok(httpcodes::HTTPOk.build().json(user_info)?)
                                },
                                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                            }
                        }).responder()
                    },
                    Err(_) => {
                            dbexecutor.send(UserInfo{
                                    user_id: "".to_string(),
                                })         
                                .from_err()
                                .and_then(|res| {
                                    match res {
                                        Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                                        Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                                    }
                                }).responder()
                    },
                }
            },
            Err(_) => {
                    dbexecutor.send(UserInfo{
                            user_id: "".to_string(),
                        })         
                        .from_err()
                        .and_then(|res| {
                            match res {
                                Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                            }
                        }).responder()
            },
        }
}

impl Handler<UserInfo> for DbExecutor {
    type Result = Result<UserInfoMsgs, Error>;
    fn handle(&mut self, user_info: UserInfo, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let user_id: i32 = user_info.user_id.parse().unwrap();
        let user_result =  users.filter(&id.eq(&user_id)).load::<User>(&self.0);
        let login_user = match user_result {
            Ok(ref user_some) => match user_some.first() {
                Some(a_user) => Some(a_user.clone()),
                None => None,
            },
            Err(_) => None,
        };
        match login_user {
            Some(login_user) => {
                    let current_user = User {
                            id: login_user.id,
                            email: login_user.email.clone(),
                            username: login_user.username.clone(),
                            password: login_user.password.clone(),
                            created_at : login_user.created_at.clone(),
                    };
                    Ok(UserInfoMsgs { 
                            status: 200,
                            message : "The  current_user info.".to_string(),
                            current_user: current_user,
                    })
            },
            None => {
                    let no_user = User {
                            id: 0,
                            email: "".to_owned(),
                            username: "".to_owned(),
                            password: "".to_owned(),
                            created_at : SystemTime::now(),
                    };
                    Ok(UserInfoMsgs { 
                            status: 400,
                            message : "error.".to_string(),
                            current_user: no_user,
                    })
            },
        }
    }
}