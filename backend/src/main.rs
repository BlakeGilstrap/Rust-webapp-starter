#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate futures;
extern crate num_cpus;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate http;
extern crate regex;

use actix::*;
use actix_web::*;
use std::env;

mod api;
mod model;
mod handler;
mod utils;

use http::header;
use actix_web::middleware::cors;
use model::db::DbExecutor;
use api::user::{ State, name, info };

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();
    let sys = actix::System::new("webapp");
    env::set_var("RUST_BACKTRACE", "1");
    let addr = SyncArbiter::start( num_cpus::get() * 3, || DbExecutor::new());
    HttpServer::new(
        move || Application::with_state(State{db: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/{name}", |r| r.f(name))
            .resource("/user/info", |r| {
                cors::Cors::build()                   
                 .allowed_origin("http://localhost:1234")
                 .allowed_methods(vec!["GET", "POST"])
                 .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                 .allowed_header(header::CONTENT_TYPE)
                 .max_age(3600)
                 .finish().expect("Can not create CORS middleware")
                 .register(r); 
                r.method(Method::POST).a(info);
            }))
        .bind("127.0.0.1:8000").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8000");
    let _ = sys.run();
}