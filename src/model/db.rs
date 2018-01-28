use actix::*;
use diesel::prelude::*;
use diesel::r2d2::*;
use std::ops::Deref;
use dotenv;


// actix-web DbExecutor
pub struct DbExecutor(pub PgConnection);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new() -> DbExecutor {
        let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        DbExecutor(PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url)))
    }
}

//  r2d2_diesel
pub type PoolDsl = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool_diesel() -> PoolDsl {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

pub struct ConnDsl(pub PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for ConnDsl {
    type Target = PgConnection;
    #[inline(always)]
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}