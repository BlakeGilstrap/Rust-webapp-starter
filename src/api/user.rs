use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use futures::future::Future;
use handler::index::State;
use model::db::DbExecutor;
use utils::token::verify_token;
use model::response::UserInfoMsgs;
use model::user::{ User, UserInfo };

impl Message for UserInfo {
    type Result = Result<UserInfoMsgs, Error>;
}

pub fn user_info(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
        let dbexecutor = req.state().db.clone();
        let header_token: Vec<_> = req.headers_mut().get("Authorization").collect();
        if header_token.len() != 1 {
            println!("The Token verify error");
        }
        let some_token = header_token[0];
        let token = &some_token[7..];
        match verify_token(token.to_string()) {
            Ok(uid) => {
                dbexecutor.send(UserInfo{
                    user_id: uid,
                })         
                .from_err()
                .and_then(|res| {
                    match res {
                        Ok(user_info) => Ok(httpcodes::HTTPOk.build().json(user_info)?),
                        Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                    }
                }).responder()
            },
            Err(_) => {
                Ok(httpcodes::HTTPInternalServerError.into())
            },
        }
}

impl Handler<UserInfo> for DbExecutor {
    type Result = Result<UserInfoMsgs, Error>;
    fn handle(&mut self, user_info: UserInfo, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let user_id: i32 = user_info.user_id.parse().unwrap();
        let user_result =  users.filter(&id.eq(&user_id)).load::<User>(&self.0);
        let current_user = match user_result {
            Ok(user_some) => match user_some.first() {
                Some(a_user) => Some(a_user.clone()),
                None => None,
            },
            Err(_) => None,
        };
        Ok(UserInfoMsgs { 
                status: 200,
                message : "The  current_user info.".to_string(),
                current_user: current_user,
        })
    }
}