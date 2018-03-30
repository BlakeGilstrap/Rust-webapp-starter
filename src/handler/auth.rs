use actix_web::*;
use futures::future::Future;
use bytes::Bytes;
use diesel;
use diesel::prelude::*;
use std::time::SystemTime;
use handler::index::State;
use bcrypt::{DEFAULT_COST, hash, verify};
use utils::token::{ self, verify_token };
use model::user::{User,NewUser,SignupUser,SigninUser};

pub fn signup(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let executor = req.state().db.clone();
    req.json()                     
       .from_err()
       .and_then(move |signup_user: SignupUser| {  
            executor.send(SignupUser{ 
                username: signup_user.username,
                email: signup_user.email,
                password: signup_user.password,
                confirm_password: signup_user.confirm_password,
            })         
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                    Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                }
            })
        }).responder()
}

pub fn signin(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let executor = req.state().db.clone();
    req.json()                     
       .from_err()
       .and_then(move |signin_user: SigninUser| {  
            executor.send(SigninUser{ 
                username: signin_user.username,
                password: signin_user.password,
            })         
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(msg) => Ok(httpcodes::HTTPOk.build().json(msg)?),
                    Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                }
            })
        }).responder()
}
