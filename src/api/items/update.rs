// create an item
use actix_web::{put, Error, HttpResponse, web};
use sea_orm::TryIntoModel;
use crate::{models::AppState, query::items};

#[put("")]
pub async fn item(data: web::Data<AppState>, payload: web::Json<entity::custom::items::UpdateItem>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = payload.into_inner();
    let items = items::update(conn, form).await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = items.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}