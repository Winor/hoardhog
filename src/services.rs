// use actix_web::{get, post, Error, HttpResponse, Responder, web};
// use sea_orm::TryIntoModel;
// use crate::{ models::AppState};
// // index
// #[get("/")]
// pub async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[get("/items")]
// pub async fn list_items(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
//     let conn = &data.conn;
//     let items = actions::get_all_items(conn).await
//      .map_err(actix_web::error::ErrorInternalServerError)?;

//       if let Some(items) = items {
//         Ok(HttpResponse::Ok().json(items))
//     } else {
//         let res = HttpResponse::NotFound().body(format!("No items found"));
//         Ok(res)
//     }
// }

// #[post("/items")]
// pub async fn create_item(data: web::Data<AppState>, payload: web::Json<entity::items::Model>) -> Result<HttpResponse, Error> {
//     let conn = &data.conn;
//     let form = payload.into_inner();
//     let items = actions::create_item(conn, form).await
//     .map_err(actix_web::error::ErrorInternalServerError)?;
//     let tst = items.try_into_model().map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(tst))
// }