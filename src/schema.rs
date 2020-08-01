table! {
    places (id) {
        id -> Uuid,
        name -> Text,
        info -> Text,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

joinable!(places -> users (created_by));

allow_tables_to_appear_in_same_query!(
    places,
    users,
);
