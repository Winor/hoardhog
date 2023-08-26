use entity::items as item;
use entity::locations as location;
use migration::DbErr;
use sea_orm::{ActiveModelTrait,IntoActiveModel,EntityTrait,DeleteResult,DatabaseConnection};
use entity::custom::items::*;
use sea_orm::LoaderTrait;

pub async fn get(conn: &DatabaseConnection) -> Result<Option<Vec<item::Model>>, sea_orm::DbErr> {
    let items: Vec<item::Model> = item::Entity::find().all(conn).await?;
    Ok(Some(items))
}


pub async fn get_all(conn: &DatabaseConnection) -> Result<Option<Vec<AllItems>>, sea_orm::DbErr> {
    let items: Vec<item::Model> = item::Entity::find().all(conn).await?;
    let locations = items.load_one(location::Entity, conn).await?.into_iter();
    let categories = items
        .load_one(entity::categories::Entity, conn)
        .await?
        .into_iter();
    let res = items
        .into_iter()
        .zip(locations)
        .zip(categories)
        .map(|((item, location), category)| AllItems {
            items: item,
            location: location,
            category: category,
        })
        .collect();
    Ok(Some(res))
}

pub async fn create(
    conn: &DatabaseConnection,
    item_data: NewItem,
) -> Result<item::ActiveModel, sea_orm::DbErr> {
    item::ActiveModel::from(item_data.into_active_model())
        .save(conn)
        .await
}

pub async fn find(conn: &DatabaseConnection, id: i32) -> Result<item::ActiveModel, sea_orm::DbErr> {
    item::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
        .map(Into::into)
}

pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<DeleteResult, sea_orm::DbErr> {
    find(&conn, id).await?.delete(conn).await
}

pub async fn update(
    conn: &DatabaseConnection,
    item_data: UpdateItem,
) -> Result<item::ActiveModel, sea_orm::DbErr> {
    //let item: item::Model = find_item(&conn, item_data.id).await.into()?;
    item::ActiveModel::from(item_data.into_active_model())
        .save(conn)
        .await
}