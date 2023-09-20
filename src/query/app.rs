use serde::{Deserialize, Serialize};
use sea_orm::{DatabaseConnection, LoaderTrait};
use super::items;

#[derive(Serialize, Deserialize)]
pub struct AllItems {
    pub item: entity::items::Model,
    pub location: Option<entity::locations::Model>,
    pub category: Option<entity::categories::Model>,
}

pub async fn item_with_related(conn: &DatabaseConnection) -> Result<Option<Vec<AllItems>>, sea_orm::DbErr> {
    let items = items::get_all(conn).await?.unwrap_or_default();
    let locations = items.load_one(entity::locations::Entity, conn).await?.into_iter();
    let categories = items
        .load_one(entity::categories::Entity, conn)
        .await?
        .into_iter();
    let res = items
        .into_iter()
        .zip(locations)
        .zip(categories)
        .map(|((item, location), category)| AllItems {
            item: item,
            location: location,
            category: category,
        })
        .collect();
    Ok(Some(res))
}