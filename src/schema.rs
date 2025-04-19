// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        text -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    recipes,
    users,
);
