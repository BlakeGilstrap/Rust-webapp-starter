use actix_web::*;
use futures::future::Future;
use bytes::Bytes;
use diesel;
use diesel::prelude::*;
use std::time::SystemTime;
use handler::index::State;
use bcrypt::{DEFAULT_COST, hash, verify};
use utils::token::{ self, verify_token };
use handler::func::{ SignupUser, SigninUser };
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
       .and_then(|signup_user: Signup| {  
            if &signup_user.password == &signup_user.confirm_password {
                use utils::schema::users::dsl::*;
                let hash_password = match hash(&signup_user.password, DEFAULT_COST) {
                    Ok(h) => h,
                    Err(_) => panic!()
                };
                let new_user = NewUser {
                    email: &signup_user.email,
                    username: &signup_user.username,
                    password: &hash_password,
                    created_at: SystemTime::now(),
                };
                diesel::insert_into(users).values(&new_user).execute(&req.state().db_pool_dsl).expect("User is  Exist!");
            }else{

            }
            Ok(httpcodes::HTTPOk.into())
       }).responder()
}
