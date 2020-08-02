# User API

## Registration

### Request

`POST /api/v1/users`

Headers:
`Content-Type: application/json`

```json
{
  "username": string,
  "email": string,
  "password": string
}
```

### Response

`201 Created`

```json
{
  "token": string,
  "username": string,
  "email": string
}
```

### Errors

| Description         | Response                          |
|---------------------|-----------------------------------|
| Invalid data        | `200 Ok` kind: "field_validation" |
| User already exists | `200 Ok` kind: "already_exists"   |
| Internal error      | `500 Internal Server Error`       |


## Login

### Request

`POST /api/v1/users/login`

Headers:
`Content-Type: application/json`

```json
{
  "username": string,
  "password": string
}
```

### Response

`200 Ok`

```json
{
  "token": string,
  "username": string,
  "email": string
}
```

### Errors

| Description         | Response                                        |
|---------------------|-------------------------------------------------|
| Invalid data        | `200 Ok` { "error": { kind: "authorization" } } |
| Internal error      | `500 Internal Server Error`                     |


## Get current user data

### Request

`GET /api/v1/user`

Headers:
`Authorization: <token>`

### Response

`200 Ok`

```json
{
  "username": string,
  "email": string
}
```

### Errors

| Description              | Response                          |
|--------------------------|-----------------------------------|
| Bad or missing JWT token | `401 Unauthorized`                |
| Internal error           | `500 Internal Server Error`       |


## Update current user data

### Request

`PUT /api/v1/user`

Headers:
`Authorization: <token>`
`Content-Type: application/json`

### Response

`200 Ok`

```json
{
  "username": string,
  "email": string
}
```

### Errors

| Description              | Response                          |
|--------------------------|-----------------------------------|
| Invalid data             | `200 Ok` kind: "field_validation" |
| Bad or missing JWT token | `401 Unauthorized`                |
| Internal error           | `500 Internal Server Error`       |



# Place API

## New

### Request

`POST /api/v1/place`

Headers:
`Authorization: <token>`
`Content-Type: application/json`

```json
{
  "name": string,
  "info": string
}
```

### Response

`201 Created`

```json
{
  "name": string,
  "info": string
}
```

### Errors

| Description    | Response                    |
|----------------|-----------------------------|
| Internal error | `500 Internal Server Error` |


## List

### Request

`GET /api/v1/places`

Query:
* `offset` - positive number; Ignore first `offset` places
* `limit` - positive number; Show only `limit` places

### Response

`201 Created`

```json
[
  {
    "name": string,
    "info": string
  }
]
```

### Errors

| Description    | Response                    |
|----------------|-----------------------------|
| Internal error | `500 Internal Server Error` |



# Errors

All errors returned with code `200 Ok` have the following format:

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
