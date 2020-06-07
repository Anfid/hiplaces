use actix_web::{web, web::Data, HttpRequest};

mod users;
mod locations;

use crate::db::Database;

pub struct AppState {
    pub db: Database,
}

async fn api(_state: Data<AppState>, _req: HttpRequest) -> &'static str {
    "Hello, World"
}

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").to(api)).service(
        web::scope("/api/v1")
            // User routes
            .service(web::resource("users").route(web::post().to(users::register)))
            .service(web::resource("users/login").route(web::post().to(users::login)))
            .service(
                web::resource("user")
                    .route(web::get().to(users::get_current))
                    .route(web::put().to(users::update)),
            )
            // TODO: Article routes
            /*
            .service(
                web::resource("locations")
                    .route(web::get().to(locations::list))
                    .route(web::post().to(locations::create)),
            )
            .service(
                web::resource("locations/{slug}")
                    .route(web::get().to(locations::get))
                    .route(web::put().to(locations::update))
                    .route(web::delete().to(locations::delete)),
            )
            .service(
                web::resource("locations/{slug}/favorite")
                    .route(web::post().to(locations::favorite))
                    .route(web::delete().to(locations::unfavorite)),
            )
            .service(
                web::resource("locations/{slug}/comments")
                    .route(web::get().to(locations::comments::list))
                    .route(web::post().to(locations::comments::add)),
            )
            .service(
                web::resource("locations/{slug}/comments/{comment_id}")
                    .route(web::delete().to(locations::comments::delete)),
            )
            */
    );
}
