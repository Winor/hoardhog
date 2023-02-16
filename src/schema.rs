// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        title -> Varchar,
        quantity -> Int4,
        unit -> Varchar,
        about -> Nullable<Text>,
        instock -> Bool,
    }
}
