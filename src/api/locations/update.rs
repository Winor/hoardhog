use actix_web::{put, Error, HttpResponse, web};
use entity::custom::locations::Update;
use sea_orm::TryIntoModel;
use crate::{models::AppState, api::locations::dbq};

#[put("")]
pub async fn update(data: web::Data<AppState>, payload: web::Json<Update>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let form = payload.into_inner();
    let items = dbq::update(conn, form).await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    let result = items.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}