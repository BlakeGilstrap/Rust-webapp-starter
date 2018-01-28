use actix_web::*;
use futures::future::Future;
use bytes::Bytes;
use diesel;
use std::time::SystemTime;
use handler::index::State;
use handler::func::{ SignupUser, SigninUser };
use model::db::ConnDsl;

#[derive(Deserialize,Serialize, Debug)]
struct Signup {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

pub fn signup(req: HttpRequest<State>, conn_dsl: ConnDsl) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.json()                     
       .from_err()
       .and_then(|signup_user: Signup| {  
            use utils::schema::users;
            if &signup_user.password == &signup_user.confirm_password {
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
                diesel::insert_into(users::table).values(&new_user).execute(&*conn_dsl).expect("User is  Exist!");
            }else{

            }
            Ok(httpcodes::HTTPOk.into())
       }).responder()
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
