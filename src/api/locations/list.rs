// list all items
use actix_web::{get, Error, HttpResponse, web};
use crate::{models::AppState, api::locations::dbq};

#[get("")]
pub async fn items(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let items = dbq::get_all(conn).await
     .map_err(actix_web::error::ErrorInternalServerError)?;

      if let Some(items) = items {
        Ok(HttpResponse::Ok().json(items))
    } else {
        let res = HttpResponse::NotFound().body(format!("Not found"));
        Ok(res)
    }
}