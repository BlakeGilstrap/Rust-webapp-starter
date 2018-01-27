use actix_web::*;
use futures::future::Future;
use bytes::Bytes;
use handler::index::State;
use handler::user::{ SignupUser, SigninUser };

#[derive(Deserialize,Serialize, Debug)]
struct MyObj {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

// work
pub fn body(mut req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    println!("======== {:?}", req.query_string());
    println!("======== {:?}", req.headers());
    req.body()                     // <- get Body future
       .limit(1024)                // <- change max size of the body to a 1kb
       .from_err()
       .and_then(|bytes: Bytes| {  // <- complete body
           println!("==== BODY ==== {:?}", bytes);
           Ok(httpcodes::HTTPOk.into())
       }).responder()
}

// error
pub fn json(mut req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()                   
       .from_err()
       .and_then(|val: MyObj| {  
           println!("==== BODY ==== {:?}", val);
           Ok(httpcodes::HTTPOk.into())
       }).responder()
}

// error
pub fn signup(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let email = &req.match_info()["email"];
    let username = &req.match_info()["username"];
    let password = &req.match_info()["password"];
    req.state().db.call_fut(SignupUser{
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

// pub fn signin(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
//     req.state().db.call_fut(SigninUser{
//             username: req.match_info()["username"].to_owned(),
//             password: req.match_info()["password"].to_owned(),
//         })
//         .from_err()
//         .and_then(|res| {
//             match res {
//                 Ok(user) => Ok(httpcodes::HTTPOk.build().json(user)?),
//                 Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
//             }
//         })
//         .responder()
// }
