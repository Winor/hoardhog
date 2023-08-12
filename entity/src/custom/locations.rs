use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::locations::ActiveModel;

// DeriveIntoActiveModel

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct New {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct Update {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}
