use actix_web::{web, web::Data, HttpRequest};

pub mod places;
pub mod users;

use crate::db::Database;

const API_DESC: &str = include_str!("../../api_description.md");

pub struct AppState {
    pub db: Database,
}

async fn api(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    API_DESC
}

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").route(web::get().to(api)))
        .service(
            web::scope("/api/v1")
                // User routes
                .service(web::resource("users").route(web::post().to(users::register)))
                .service(web::resource("users/login").route(web::post().to(users::login)))
                .service(
                    web::resource("user")
                        .route(web::get().to(users::get_current))
                        .route(web::put().to(users::update)),
                )
                .service(
                    web::resource("place")
                        .route(web::post().to(places::create)),
                )
                .service(web::resource("places").route(web::get().to(places::list))),
        );
}
