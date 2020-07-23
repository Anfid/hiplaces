#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    App, HttpServer,
};
use std::{convert::Into, env};

mod app;
mod db;
mod middleware;
mod models;
mod result;
mod schema;
mod util;

use app::AppState;
use db::Database;
use result::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "restful-image=debug,actix_web=info");
    }
    env_logger::init();

    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    let db = Database::init(database_url)?;

    let server = HttpServer::new(move || {
        let cors = /*match frontend_origin {
            Some(ref origin) => Cors::new()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600)
                .finish(),
            None => Cors::new()
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600)
                .finish(),
        }*/ Cors::new().finish();

        App::new()
            .data(AppState { db: db.clone() })
            .wrap(Logger::default())
            .wrap(cors)
            .configure(app::routes)
    })
    .bind(&bind_address)
    .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address));

    info!("You can access the server at {}", bind_address);

    server.run().await.map_err(Into::into)
}
