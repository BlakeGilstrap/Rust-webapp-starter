use actix::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new() -> DbExecutor {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DbExecutor(PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url)))
    }
}

