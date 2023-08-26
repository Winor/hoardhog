// list all items
use actix_web::{get, Error, web, Responder};
use crate::{models::AppState, api::items::dbq};

#[get("")]
pub async fn items(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let items = dbq::get_all(conn).await
     .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(Some(web::Json(items)))
}