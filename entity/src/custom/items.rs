use serde::{Deserialize, Serialize};
use sea_orm::{entity::prelude::*};
use crate::items::ActiveModel;

// DeriveIntoActiveModel

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct NewItem {
    pub name: String,
    pub quantity: Option<i32>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub instock: Option<bool>,
}

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct UpdateItem {
    pub id: i32,
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub instock: Option<bool>,
}
