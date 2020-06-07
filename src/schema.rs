table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
