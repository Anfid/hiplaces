use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde_derive::{Deserialize, Serialize};

use super::AppState;
use crate::auth::Claims;
use crate::models;
use crate::result::{Error, Result};

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
    text: String,
}

#[derive(Deserialize)]
pub struct GetPlace {
    name: String,
}

impl From<CreatePlace> for models::place::NewPlace {
    fn from(place: CreatePlace) -> models::place::NewPlace {
        models::place::NewPlace {
            name: place.name,
            info: place.text,
        }
    }
}

// TODOs:
// * Store location. Consider using https://github.com/Boscop/diesel-geography
// * Image view
pub async fn create(
    state: Data<AppState>,
    place: Json<CreatePlace>,
    _claims: Claims,
) -> Result<HttpResponse> {
    let place = place.into_inner();
    state
        .db
        .new_place(place.into())
        .map(|place| HttpResponse::Ok().json(PlaceResponse::from(place)))
}

pub async fn list(state: Data<AppState>, place: Json<GetPlace>) -> Result<HttpResponse> {
    let place = place.into_inner();
    match state.db.get_place(place.name) {
        Ok(place) => Ok(HttpResponse::Ok().json(PlaceResponse::from(place))),
        Err(e) => Err(e),
    }
}
