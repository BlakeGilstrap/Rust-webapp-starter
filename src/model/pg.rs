use actix::*;
use r2d2::*;
use r2d2_postgres::{ PostgresConnectionManager, TlsMode };
use dotenv;

//  r2d2_postgres
pub struct PoolPg(pub Pool<PostgresConnectionManager>);

impl Actor for PoolPg {
    type Context = SyncContext<Self>;
}

impl PoolPg {
    pub fn new() -> PoolPg {
        let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
        PoolPg(Pool::new(manager).unwrap().clone())
    }
}
