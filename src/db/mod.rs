use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use crate::result::Result;

pub mod user;
pub mod place;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: DbPool,
}

impl Database {
    pub fn init(url: String) -> Result<Database> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::builder().build(manager)?;
        Ok(Database { pool })
    }
}
