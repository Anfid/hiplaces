use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::schema::places;

#[derive(Debug, Queryable)]
pub struct Place {
    pub id: Uuid,
    pub name: String,
    pub info: String,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "places"]
pub struct NewPlace {
    pub name: String,
    pub info: String,
}
