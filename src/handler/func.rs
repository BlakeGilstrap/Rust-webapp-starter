use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use utils::token::verify_token;
use chrono::{DateTime,Utc};
use utils::token;
use std::time::SystemTime;
use bcrypt::{DEFAULT_COST, hash, verify};

use model::user::{User,NewUser};
use model::db::DbExecutor;

#[derive(Deserialize,Serialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
pub struct Message {
    pub msg: String,
}
impl ResponseType for SignupUser {
    type Item = Message;
    type Error = Error;
}

impl Handler<SignupUser> for DbExecutor {
    type Result = MessageResult<SignupUser>;
    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result {
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
                println!("===========new_user: {:?}============", new_user);
                diesel::insert_into(users).values(&new_user).execute(&self.0).expect("Error inserting person");

                Ok(Message { msg: "Successful".to_string()})
        }else{
            Ok(Message { msg: "Something wrong".to_string()})
        }
    }
}