// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        content -> Varchar,
        is_finished -> Bool,
    }
}
