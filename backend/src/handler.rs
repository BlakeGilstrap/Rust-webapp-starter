use diesel;
use uuid;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use chrono::{DateTime,Utc};
use bcrypt::{DEFAULT_COST, hash, verify};

use schema;
use db::DbExecutor;
use model::{User,NewUser};

pub struct UserInfo {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl ResponseType for UserInfo {
    type Item = User;
    type Error = Error;
}

impl Handler<UserInfo> for DbExecutor {
    type Result = MessageResult<UserInfo>;
    fn handle(&mut self, user_info: UserInfo, _: &mut Self::Context) -> Self::Result  {
        use self::schema::users::dsl::*;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let hash_password = match hash(&user_info.password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => panic!()
        };
        let new_user = NewUser {
                id: &uuid,
                email: &user_info.email,
                username: &user_info.username,
                password: &hash_password,
        };        
        diesel::insert_into(users).values(&new_user).execute(&self.0).expect("Error inserting person");
        let mut items = users.filter(id.eq(&uuid)).load::<User>(&self.0).expect("Error loading person");
        
        Ok(items.pop().unwrap())
    }
}