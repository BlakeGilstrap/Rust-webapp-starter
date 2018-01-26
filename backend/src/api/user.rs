use actix::*;
use actix_web::*;
use std::path::Path;
use model::db::DbExecutor;
use futures::future::Future;
use handler::user::UserInfo;

pub struct State {
    pub db: SyncAddress<DbExecutor>,
}

pub fn name(req: HttpRequest<State>) -> String {
    format!("Hello {}!", &req.match_info()["name"])
}

pub fn info(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let email = &req.match_info()["email"];
    let username = &req.match_info()["username"];
    let password = &req.match_info()["password"];
    req.state().db.call_fut(UserInfo{
            email: email.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(user) => Ok(httpcodes::HTTPOk.build().json(user)?),
                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
            }
        })
        .responder()
}

