// delete an item
use actix_web::{delete, Error, HttpResponse, web};
use crate::{models::AppState, query::locations};

#[delete("{id}")]
pub async fn delete(data: web::Data<AppState>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    locations::delete(conn, id).await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(format!("Deleted item with ID {}", id)))
}