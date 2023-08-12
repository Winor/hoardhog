use actix_web::web::Json;
use entity::items as item;
use entity::locations as location;
use migration::DbErr;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::Insert;
use sea_orm::IntoActiveModel;

// use super::schemas::NewItem;

use entity::custom::items::*;
use sea_orm::Set;

// type DbError = Box<dyn std::error::Error + Send + Sync>;

pub async fn get_all_items(conn: &DatabaseConnection) -> Result<Option<Vec<item::Model>>, sea_orm::DbErr> {
    let items: Vec<item::Model> = item::Entity::find().all(conn).await?;
    Ok(Some(items))
}

pub async fn get_all_items_rl(conn: &DatabaseConnection) -> Result<Option<Vec<(entity::items::Model, Vec<entity::locations::Model>)>>, sea_orm::DbErr> {
    // let items: Vec<item> = Item::find().all(conn).await?;
    // Ok(Some(items))
    let cake_with_fruits: Vec<(item::Model, Vec<location::Model>)> = item::Entity::find()
    .find_with_related(location::Entity)
    .all(conn)
    .await?;
    Ok(Some(cake_with_fruits))
}

pub async fn create_item(conn: &DatabaseConnection, item_data: NewItem) -> Result<item::ActiveModel, sea_orm::DbErr> {
    item::ActiveModel::from(item_data.into_active_model()).save(conn).await
}

pub async fn find_item(conn: &DatabaseConnection, id: i32) -> Result<item::ActiveModel, sea_orm::DbErr> {
    item::Entity::find_by_id(id).one(conn).await?.ok_or(DbErr::Custom("Cannot find post.".to_owned())).map(Into::into)
}

pub async fn delete_item(conn: &DatabaseConnection, id: i32) -> Result<DeleteResult, sea_orm::DbErr> {
    find_item(&conn, id).await?.delete(conn).await
}

pub async fn update_item(conn: &DatabaseConnection, item_data: UpdateItem) -> Result<item::ActiveModel, sea_orm::DbErr> {
    //let item: item::Model = find_item(&conn, item_data.id).await.into()?;
    item::ActiveModel::from(item_data.into_active_model()).save(conn).await
}