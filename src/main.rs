#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate bytes;
extern crate futures;
extern crate num_cpus;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;
extern crate bcrypt;
extern crate http;
extern crate ring;
extern crate data_encoding;
extern crate regex;

use actix::*;
use actix_web::*;
use std::env;

mod api;
mod handler;
mod model;
mod utils;

use model::db::DbExecutor;
use utils::cors;
use handler::index::{ State, home, path };
use handler::auth::{ signup, signin };

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("webapp");
    env::set_var("RUST_BACKTRACE", "1");
    let addr = SyncArbiter::start( num_cpus::get() * 3, || DbExecutor::new());
    HttpServer::new(
        move || Application::with_state(State{db: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(home))
            .resource(r"/a/{tail:.*}", |r| r.f(path))
            .resource("/user/signup", |r| {
                cors::options().register(r);
                r.method(Method::POST).a(body);
            })
            // .resource("/user/signin", |r| {
            //     r.middleware(cors::options());
            //     r.method(Method::POST).a(signin);
            // })
            .handler("/", fs::StaticFiles::new("public", true)))
        .bind("127.0.0.1:8000").unwrap()
        .start();

    let _ = sys.run();
}