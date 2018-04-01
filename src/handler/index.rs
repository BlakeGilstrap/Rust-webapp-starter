use actix::*;
use actix_web::*;
use std::path::Path;
use model::db::DbExecutor;
use model::pg::PoolPg;

pub struct State {
    pub db: Addr<Syn, DbExecutor>,
    pub db_pg: Addr<Syn, PoolPg>,
}

pub fn home(_req: HttpRequest<State>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(Path::new("public/index.html"))?)
}
pub fn path(_req: HttpRequest<State>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(Path::new("public/index.html"))?)
}