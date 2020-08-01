use diesel::prelude::*;
use std::convert::Into;

use super::Database;
use crate::models::place::{NewPlace, Place};
use crate::result::{Error, Result};

impl Database {
    pub fn new_place(&self, place: NewPlace) -> Result<Place> {
        use crate::schema::places::dsl::*;

        let conn = self.pool.get()?;

        diesel::insert_into(places)
            .values(place)
            .get_result::<Place>(&conn)
            .map_err(Into::into)
    }

    pub fn get_place(&self, place_name: String) -> Result<Place> {
        use crate::schema::places::dsl::*;

        let conn = &self.pool.get()?;

        let _place: Place = places.filter(name.eq(place_name)).first(conn)?;
        Err(Error::NotImplemented)
    }
}
