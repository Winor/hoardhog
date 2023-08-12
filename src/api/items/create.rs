// create an item
use actix_web::{post, Error, HttpResponse, web};
use sea_orm::TryIntoModel;
use crate::{models::AppState, api::items::{dbq}};
use entity::custom::items::*;

#[post("")]
pub async fn item(data: web::Data<AppState>, payload: web::Json<NewItem>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = payload.into_inner();
    let items = dbq::create(conn, form).await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = items.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}