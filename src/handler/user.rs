use diesel;
use uuid;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use utils::token::verify_token;
use chrono::{DateTime,Utc};
use utils::token;
use bcrypt::{DEFAULT_COST, hash, verify};

use utils::schema;
use model::db::DbExecutor;
use model::user::{User,NewUser};

pub struct SignupUser {
    pub email: String,
    pub username: String,
    pub password: String,
}
pub struct SigninUser {
    pub username: String,
    pub password: String,
}
impl ResponseType for SignupUser {
    type Item = User;
    type Error = Error;
}
impl ResponseType for SigninUser {
    type Item = User;
    type Error = Error;
}

impl Handler<SignupUser> for DbExecutor {
    type Result = MessageResult<SignupUser>;
    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result  {
        use self::schema::users::dsl::*;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let hash_password = match hash(&signup_user.password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => panic!()
        };
        let new_user = NewUser {
                id: &uuid,
                email: &signup_user.email,
                username: &signup_user.username,
                password: &hash_password,
        };        
        diesel::insert_into(users).values(&new_user).execute(&self.0).expect("Error inserting person");
        let mut items = users.filter(id.eq(&uuid)).load::<User>(&self.0).expect("Error loading person");
        
        Ok(items.pop().unwrap())
    }
}

// impl Handler<SigninUser> for DbExecutor {
//     type Result = MessageResult<SigninUser>;
//     fn handle(&mut self, signin_user: SigninUser, _: &mut Self::Context) -> Self::Result  {
//         use self::schema::users::dsl::*;
//         let user_result =  users.filter(username.eq(&signin_user.username)).load::<User>(&self.0).expect("Error loading person");
//         let login_user = match user_result {
//             Ok(user_some) => match user_some.first() {
//                 Some(a_user) => Some(a_user.clone()),
//                 None => None,
//             },
//             Err(_) => None,
//         };
//         match login_user {
//             Some(login_user) => {
//                 match verify(&signin_user.password, &login_user.password) {
//                     Ok(valid) => {
//                         let user_id = login_user.id.to_string();
//                         let token = token::generate_token(user_id).unwrap();

//                         let mut items = users.filter(username.eq(&signin_user.username)).load::<User>(&self.0).expect("Error loading person");
//                         Ok(items.pop().unwrap())
//                     },
//                     Err(_) => {
//                         let mut items = users.filter(username.eq(&signin_user.username)).load::<User>(&self.0).expect("Error loading person");
//                         Ok(items.pop().unwrap())
//                     },
//                 }
//             },
//             None => {
//                 let mut items = users.filter(username.eq(&signin_user.username)).load::<User>(&self.0).expect("Error loading person");
//                 Ok(items.pop().unwrap())
//             }
//         }
        
//     }
// }