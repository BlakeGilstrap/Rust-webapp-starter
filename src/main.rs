#![allow(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
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
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use actix::*;
use actix_web::*;

mod api;
mod handler;
mod model;
mod utils;

use model::db::DbExecutor;
use utils::cors;
use handler::index::{ State, home, path };
use handler::auth::{ signup, signin };
use api::article::{ article_list, article_new };

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("webapp");
    let addr = SyncArbiter::start( num_cpus::get() * 4, || DbExecutor::new());
    // let addr_pg = SyncArbiter::start( num_cpus::get() * 4, || PoolPg::new());
    HttpServer::new(
        move || Application::with_state(State{
                db: addr.clone(),
                // db_pg:addr_pg.clone(),
            })
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(home))
            .resource(r"/a/{tail:.*}", |r| r.f(path))
            .resource("/user/signup", |r| {
                cors::options().register(r);
                r.method(Method::POST).a(signup);
            })
            .resource("/user/signin", |r| {
                cors::options().register(r);
                r.method(Method::POST).a(signin);
            })
            .resource("/api/article_list", |r| {
                cors::options().register(r);
                r.method(Method::GET).a(article_list);
            })
            .resource("/api/article_new", |r| {
                cors::options().register(r);
                r.method(Method::POST).a(article_new);
            })
            .handler("/", fs::StaticFiles::new("public", true)))
        .bind("127.0.0.1:8000").unwrap()
        .shutdown_timeout(2)
        .start();

    let _ = sys.run();
}