use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::items::ActiveModel;

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
