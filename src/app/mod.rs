use actix_web::{web, web::Data, HttpRequest};

pub mod places;
pub mod users;

use crate::db::Database;

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

const API_DESC: &'static str = r#"
# API

## Registration

### Request

POST /api/v1/users

Content-Type: application/json

```json
{
  "username": string,
  "email": string,
  "password": string
}
```

### Response

201 Created

```json
{
  "token": string
  "username": string,
  "email": string
}
```

### Errors

* Invalid data -> 200 Ok kind: "field_validation"
* User already exists -> 200 Ok kind: "already_exists"
* Internal error -> 500 Internal Server Error


## Login

### Request

POST /api/v1/users/login

Content-Type: application/json

```json
{
  "username": string,
  "password": string
}
```

### Response

200 Ok

```json
{
  "token": string
  "username": string,
  "email": string
}
```

### Errors

* Invalid data -> 200 Ok { "error": { kind: "authorization" } }
* Internal error -> 500 Internal Server Error


## Get current user data

### Request

GET /api/v1/user

Authorization: <token>

### Response

501 Not Implemented

### Errors

* Bad or missing JWT token -> 401 Unauthorized


## Update current user data

### Request

PUT /api/v1/user

Authorization: <token>

### Response

501 Not Implemented

### Errors

* Bad or missing JWT token -> 401 Unauthorized


# Errors

Errors have the following format:

```json
{
  "error": {
    "kind": "<error_kind>",
    "info": <error_data>
  }
}
```

Possible `error_kind` variants and structure of corresponding `error_data`:
* "already_exists": none
* "not_found": none
* "authorization": none
* "field_validation": [ "<field_name>": <field_error>, ..." ]

`field_error` structure:

```json
{
  "code": string,
  "message": string,
  "params": dict
}
```
"#;
