//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub category_id: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub location_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Categories,
    #[sea_orm(has_many = "super::item_field::Entity")]
    ItemField,
    #[sea_orm(has_many = "super::item_group::Entity")]
    ItemGroup,
    #[sea_orm(
        belongs_to = "super::locations::Entity",
        from = "Column::LocationId",
        to = "super::locations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Locations,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::item_field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemField.def()
    }
}

impl Related<super::item_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemGroup.def()
    }
}

impl Related<super::locations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Locations.def()
    }
}

impl Related<super::field::Entity> for Entity {
    fn to() -> RelationDef {
        super::item_field::Relation::Field.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::item_field::Relation::Items.def().rev())
    }
}

impl Related<super::groups::Entity> for Entity {
    fn to() -> RelationDef {
        super::item_group::Relation::Groups.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::item_group::Relation::Items.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
