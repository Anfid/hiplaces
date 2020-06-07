use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use diesel::prelude::*;
use std::convert::Into;

use crate::models::user::{NewUser, User};
use crate::result::Result;

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

impl Database {
    pub fn register_user(&self, user: NewUser) -> Result<User> {
        use crate::schema::users::dsl::*;

        let conn = self.pool.get()?;

        diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&conn)
            .map_err(Into::into)
    }
}
