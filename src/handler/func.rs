use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use utils::token::verify_token;
use chrono::{DateTime,Utc};
use utils::token;
use std::time::SystemTime;
use bcrypt::{DEFAULT_COST, hash, verify};

use utils::schema;
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