use diesel::prelude::*;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use libreauth::pass::HashBuilder;
use std::convert::Into;

use crate::app::users::LoginUser;
use crate::models::user::{NewUser, User};
use crate::result::{Error, Result};
use crate::util::{HASHER, PWD_SCHEME_VERSION};

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

    pub fn login_user(&self, user: LoginUser) -> Result<User> {
        use crate::schema::users::dsl::*;

        let conn = &self.pool.get()?;

        let stored_user: User = users.filter(username.eq(user.username)).first(conn)?;
        let checker = HashBuilder::from_phc(&stored_user.password)?;

        if checker.is_valid(&user.password) {
            if checker.needs_update(Some(PWD_SCHEME_VERSION)) {
                let new_password = HASHER.hash(&user.password)?;
                match diesel::update(users.find(stored_user.id))
                    .set(password.eq(new_password))
                    .get_result::<User>(conn)
                {
                    Ok(user) => Ok(user.into()),
                    Err(e) => Err(e.into()),
                }
            } else {
                Ok(stored_user.into())
            }
        } else {
            Err(Error::Authorization(String::from("Invalid password")))
        }
    }
}
