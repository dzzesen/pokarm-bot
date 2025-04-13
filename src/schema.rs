diesel::table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        text -> Varchar,
        updated_at -> Timestamp,
    }
}
