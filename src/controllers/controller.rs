// use std::fmt::Debug;

use actix_web::HttpResponse;
use diesel::{r2d2::{ConnectionManager, PooledConnection}, PgConnection};

#[allow(dead_code)]
pub type PoolDbController = PooledConnection<ConnectionManager<PgConnection>>;

pub trait Controller {
    fn new(&self,db: PoolDbController) -> HttpResponse;
    fn validator<'a>(&self, db:&'a mut PoolDbController) -> Result<(), String>;
}