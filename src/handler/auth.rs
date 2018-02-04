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
    req.json()                     
       .from_err()
       .and_then(move |signup_user: Signup| {  
            println!("============{:?}===========",signup_user);
            {
                println!("============111===========");
                &req.state().db.call_fut(SignupUser{ 
                username: signup_user.username.to_string(),
                email: signup_user.email.to_string(),
                password: signup_user.password.to_string(),
                confirm_password: signup_user.confirm_password.to_string(),
                });
            }
           
            println!("============end===========");
            Ok(httpcodes::HTTPOk.into())
       }).responder()
}
