// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    item_group (item_id, group_id) {
        item_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    item_location (item_id, location_id) {
        item_id -> Int4,
        location_id -> Int4,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        quantity -> Int4,
        unit -> Varchar,
        description -> Nullable<Text>,
        instock -> Bool,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tag_group (tag_id, group_id) {
        tag_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    tag_item (tag_id, item_id) {
        tag_id -> Int4,
        item_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(item_group -> groups (group_id));
diesel::joinable!(item_group -> items (item_id));
diesel::joinable!(item_location -> items (item_id));
diesel::joinable!(item_location -> locations (location_id));
diesel::joinable!(tag_group -> groups (group_id));
diesel::joinable!(tag_group -> tags (tag_id));
diesel::joinable!(tag_item -> items (item_id));
diesel::joinable!(tag_item -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    item_group,
    item_location,
    items,
    locations,
    tag_group,
    tag_item,
    tags,
);
