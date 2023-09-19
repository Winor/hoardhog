use entity::custom::locations;
use entity::locations as location;
use migration::DbErr;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;

pub async fn get_all(conn: &DatabaseConnection) -> Result<Option<Vec<location::Model>>, sea_orm::DbErr> {
    let list: Vec<location::Model> = location::Entity::find().all(conn).await?;
    Ok(Some(list))
}

pub async fn create(conn: &DatabaseConnection, data: locations::New) -> Result<location::ActiveModel, sea_orm::DbErr> {
    location::ActiveModel::from(data.into_active_model()).save(conn).await
}

pub async fn find(conn: &DatabaseConnection, id: i32) -> Result<location::ActiveModel, sea_orm::DbErr> {
    location::Entity::find_by_id(id).one(conn).await?.ok_or(DbErr::Custom("Cannot find location.".to_owned())).map(Into::into)
}

pub async fn delete(conn: &DatabaseConnection, id: i32) -> Result<DeleteResult, sea_orm::DbErr> {
    find(&conn, id).await?.delete(conn).await
}

pub async fn update(conn: &DatabaseConnection, data: locations::Update) -> Result<location::ActiveModel, sea_orm::DbErr> {
    //let item: item::Model = find_item(&conn, item_data.id).await.into()?;
    location::ActiveModel::from(data.into_active_model()).save(conn).await
}