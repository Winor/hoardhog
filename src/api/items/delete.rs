// delete an item
use actix_web::{delete, Error, HttpResponse, web};
use crate::{models::AppState, api::items::{dbq}};

#[delete("{id}")]
pub async fn item(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    dbq::delete_item(conn, id).await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(format!("Deleted item with ID {}", id)))
}