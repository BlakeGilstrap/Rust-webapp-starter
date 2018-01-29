use actix::*;
use actix_web::*;
use std::path::Path;
use model::db::ConnDsl;

pub struct State {
    pub db_pool_dsl: SyncAddress<ConnDsl>,
}

pub fn home(_req: HttpRequest<State>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(Path::new("public/index.html"))?)
}
pub fn path(_req: HttpRequest<State>) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(Path::new("public/index.html"))?)
}