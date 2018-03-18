use actix_web::*;
use futures::future::Future;
use bytes::Bytes;
use diesel;
use diesel::prelude::*;
use std::time::SystemTime;
use handler::index::State;
use bcrypt::{DEFAULT_COST, hash, verify};
use utils::token::{ self, verify_token };
use handler::func::{ SignupUser};
use model::user::{User,NewUser};

#[derive(Deserialize,Serialize, Debug)]
struct Signup {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

pub fn signup(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let executor = req.state().db.clone();
    req.json()                     
       .from_err()
       .and_then(move |signup_user: Signup| {  
            executor.send(SignupUser{ 
                username: signup_user.username,
                email: signup_user.email,
                password: signup_user.password,
                confirm_password: signup_user.confirm_password,
            })                
            .from_err()
        })
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(httpcodes::HTTPOk.build().body(msg)?),
                Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
            }
            // Ok(httpcodes::HTTPOk.into())
        }).responder()
}
