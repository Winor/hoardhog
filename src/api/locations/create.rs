// create an item
use actix_web::{post, Error, HttpResponse, web};
use sea_orm::TryIntoModel;
use crate::{models::AppState, api::locations::dbq};
use entity::custom::locations;

#[post("")]
pub async fn create(data: web::Data<AppState>, payload: web::Json<locations::New>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = payload.into_inner();
    let create = dbq::create(conn, form).await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = create.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}