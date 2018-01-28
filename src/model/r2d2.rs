use r2d2::*;
use r2d2_postgres::{ PostgresConnectionManager, TlsMode };
use postgres::Connection;
use std::ops::Deref;

use dotenv;

//  r2d2_postgres
pub type PoolPg = Pool<PostgresConnectionManager>;

pub fn init_pool_pg() -> PoolPg {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    Pool::builder().build(manager).expect("Failed to create pool.")
}

pub struct ConnPg(pub PooledConnection<PostgresConnectionManager>);

impl Deref for ConnPg {
    type Target = Connection;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}