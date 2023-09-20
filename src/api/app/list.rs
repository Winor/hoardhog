// list all items
use actix_web::{get, Error, HttpResponse, web};
use crate::{models::AppState, query::locations};

#[get("")]
pub async fn list(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let items = locations::get_all(conn).await
     .map_err(actix_web::error::ErrorInternalServerError)?;

      if let Some(items) = items {
        Ok(HttpResponse::Ok().json(items))
    } else {
        let res = HttpResponse::NotFound().body(format!("Not found"));
        Ok(res)
    }
}