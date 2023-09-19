// list all items
use actix_web::{get, Error, web, Responder};
use crate::{models::AppState, query::items};

#[get("")]
pub async fn item(data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let items = items::get_all(conn).await
     .map_err(actix_web::error::ErrorInternalServerError)?;
        Ok(Some(web::Json(items)))
}