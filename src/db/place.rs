use diesel::prelude::*;
use std::convert::Into;

use super::Database;
use crate::models::place::{NewPlace, Place};
use crate::result::Result;

impl Database {
    pub fn new_place(&self, place: NewPlace) -> Result<Place> {
        use crate::schema::places::dsl::*;

        let conn = self.pool.get()?;

        diesel::insert_into(places)
            .values(place)
            .get_result::<Place>(&conn)
            .map_err(Into::into)
    }

    pub fn get_places(&self, offset: i64, limit: Option<i64>) -> Result<Vec<Place>> {
        use crate::schema::places::dsl::*;

        let conn = &self.pool.get()?;

        let mut res = places.offset(offset).into_boxed();

        if let Some(limit) = limit {
            res = places.limit(limit).into_boxed();
        }

        res.load::<Place>(conn).map_err(Into::into)
    }
}
