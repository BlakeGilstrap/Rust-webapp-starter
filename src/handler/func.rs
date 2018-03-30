use diesel;
use actix::*;
use actix_web::*;
use diesel::prelude::*;
use utils::token::verify_token;
use utils::token;
use std::time::SystemTime;
use bcrypt::{DEFAULT_COST, hash, verify};

use model::user::{User,NewUser,SignupUser,SigninUser};
use model::response::{ SignupMsgs, SigninMsgs };
use model::db::DbExecutor;

impl Message for SignupUser {
    type Result = Result<SignupMsgs, Error>;
}
impl Message for SigninUser {
    type Result = Result<SigninMsgs, Error>;
}

impl Handler<SignupUser> for DbExecutor {
    type Result = Result<SignupMsgs, Error>;
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
                diesel::insert_into(users).values(&new_user).execute(&self.0).expect("Error inserting person");
                Ok(SignupMsgs { 
                        status: 200,
                        message : "Successful Signup.".to_string(),
                })
        }else{
            Ok(SignupMsgs { 
                    status: 400,
                    message : "failed Signup.".to_string(),
            })
        }
    }
}

impl Handler<SigninUser> for DbExecutor {
    type Result = Result<SigninMsgs, Error>;
    fn handle(&mut self, signin_user: SigninUser, _: &mut Self::Context) -> Self::Result {
        use utils::schema::users::dsl::*;
        let user_result =  users.filter(&username.eq(&signin_user.username)).load::<User>(&self.0);
        let login_user = match user_result {
            Ok(ref user_some) => match user_some.first() {
                Some(a_user) => Some(a_user.clone()),
                None => None,
            },
            Err(_) => None,
        };
        let null = User {
                id: 0,
                email: "".to_owned(),
                username: "".to_owned(),
                password: "".to_owned(),
                created_at : SystemTime::now(),
        };
        match login_user {
            Some(login_user) => {
                match verify(&signin_user.password, &login_user.password) {
                    Ok(valid) => {
                        let user_id = login_user.id.to_string();
                        let token = token::generate_token(user_id).unwrap();
                        Ok(SigninMsgs { 
                            status: 200,
                            token: token,
                            signin_user: login_user,
                            message : "Succesfully signin.".to_string(),
                        })
                    },
                    Err(_) => {
                        Ok(SigninMsgs { 
                            status: 400,
                            token: "".to_owned(),
                            signin_user: null,
                            message : "Incorrect Password.".to_string(),
                        })
                    },
                }
            },
            None => {
                Ok(SigninMsgs { 
                        status: 400,
                        token: "".to_owned(),
                        signin_user: null,
                        message : "Signin failure.".to_string(),
                })
            }
        }
    }
}