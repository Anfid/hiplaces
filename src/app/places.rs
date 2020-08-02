use actix_web::{
    web::{Data, Json, Query},
    HttpResponse,
};
use serde_derive::{Deserialize, Serialize};

use super::AppState;
use crate::auth::Claims;
use crate::models;
use crate::result::Result;

#[derive(Serialize)]
struct PlaceResponse {
    name: String,
    info: String,
}

impl From<models::place::Place> for PlaceResponse {
    fn from(place: models::place::Place) -> PlaceResponse {
        PlaceResponse {
            name: place.name,
            info: place.info,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePlace {
    name: String,
    info: String,
}

#[derive(Deserialize)]
pub struct GetPlacesQuery {
    offset: Option<usize>,
    limit: Option<usize>,
}

// TODOs:
// * Store location. Consider using https://github.com/Boscop/diesel-geography
// * Image view
pub async fn create(
    state: Data<AppState>,
    claims: Claims,
    Json(place): Json<CreatePlace>,
) -> Result<HttpResponse> {
    state
        .db
        .new_place(models::place::NewPlace {
            name: place.name,
            info: place.info,
            created_by: claims.id,
        })
        .map(|place| HttpResponse::Created().json(PlaceResponse::from(place)))
}

pub async fn list(
    state: Data<AppState>,
    Query(query): Query<GetPlacesQuery>,
) -> Result<HttpResponse> {
    match state.db.get_places(
        query.offset.unwrap_or(0) as i64,
        query.limit.map(|n| n as i64),
    ) {
        Ok(places) => Ok(HttpResponse::Ok().json(
            places
                .into_iter()
                .map(PlaceResponse::from)
                .collect::<Vec<_>>(),
        )),
        Err(e) => Err(e),
    }
}
