use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::items::ActiveModel;
use crate::item_group::Entity;
use sea_orm::{FromQueryResult, RelationTrait};
use sea_orm::sea_query;

#[derive(Serialize, Deserialize)]
pub struct AllItems {
    #[serde(flatten)]
    pub items: crate::items::Model,
    pub location: Option<crate::locations::Model>,
    pub category: Option<crate::categories::Model>,
}

#[derive(FromQueryResult)]
pub struct Item {
    pub id: i32,
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct NewItem {
    pub name: String,
    pub quantity: Option<i32>,
    pub description: Option<String>,
    pub location_id: Option<i32>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct UpdateItem {
    pub id: i32,
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub description: Option<String>,
    pub location_id: Option<i32>,
    pub category_id: Option<i32>,
}
