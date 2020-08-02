use diesel::prelude::*;
use libreauth::pass::HashBuilder;
use std::convert::Into;
use uuid::Uuid;

use super::Database;
use crate::app::users::LoginUser;
use crate::auth::{HASHER, PWD_SCHEME_VERSION};
use crate::models::user::*;
use crate::result::{Error, Result};

#[allow(unused)]
pub enum UserIdentifier {
    Uuid(Uuid),
    Username(String),
    Email(String),
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
                diesel::update(users.find(stored_user.id))
                    .set(password.eq(new_password))
                    .get_result::<User>(conn)
                    .map_err(Into::into)
            } else {
                Ok(stored_user)
            }
        } else {
            Err(Error::Authorization)
        }
    }

    pub fn update_user(&self, user: UpdateUser) -> Result<User> {
        let conn = self.pool.get()?;

        diesel::update(&user)
            .set(&user)
            .get_result::<User>(&conn)
            .map_err(Into::into)
    }

    pub fn get_user(&self, user_id: UserIdentifier) -> Result<User> {
        use crate::schema::users::dsl::*;
        use UserIdentifier::*;

        let conn = self.pool.get()?;

        match user_id {
            Uuid(uid) => users
                .filter(id.eq(uid))
                .get_result::<User>(&conn)
                .map_err(Into::into),
            Username(n) => users
                .filter(username.eq(n))
                .get_result::<User>(&conn)
                .map_err(Into::into),
            Email(e) => users
                .filter(email.eq(e))
                .get_result::<User>(&conn)
                .map_err(Into::into),
        }
    }
}
