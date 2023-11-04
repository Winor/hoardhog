// find an item
use actix_web::{get, Error, web, Responder};
use crate::{models::AppState, query::items};
use sea_orm::TryIntoModel;

#[get("{id}")]
pub async fn item(data: web::Data<AppState>, id: web::Path<i32>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let item = items::find(conn, id).await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = item.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Some(web::Json(result)))
}