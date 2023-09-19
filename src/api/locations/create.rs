// create an item
use actix_web::{post, Error, HttpResponse, web};
use sea_orm::TryIntoModel;
use crate::{models::AppState, query::locations};
use entity::custom::locations::New;

#[post("")]
pub async fn create(data: web::Data<AppState>, payload: web::Json<New>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = payload.into_inner();
    let create = locations::create(conn, form).await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = create.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}