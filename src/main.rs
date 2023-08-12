mod services;
mod models;
mod api;
use dotenvy::dotenv;
use models::AppState;
use sea_orm::{Database, DatabaseConnection };
use std::env;
use actix_web::{middleware, web, HttpServer, App};
use migration::{Migrator, MigratorTrait};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // set up database connection pool
    let conn_spec = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn: DatabaseConnection  = Database::connect(conn_spec).await.unwrap();
    Migrator::up(&conn, None).await.expect("Could not migrate db");
    let state = AppState {conn};

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(api::init_routes)
            // .service(delete_items)
            // .service(edit_items)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}


// #[macro_use] extern crate rocket;
// // #[macro_use] extern crate rocket_sync_db_pools;
// // #[macro_use] extern crate diesel;

// // use rocket_sync_db_pools::{database, diesel};
// // mod routes;
// mod models;
// mod db;
// mod schema;
// use self::db::PgDbConn;
// use self::models::*;
// use rocket::figment::{value::{Map, Value}, util::map};
// use dotenvy::dotenv;
// use std::env;

// #[launch]
// pub fn rocket() -> _ {
//     dotenv().ok();
//     let db: Map<_, Value> = map! {
//         "url" => env::var("DATABASE_URL").expect("DATABASE_URL must be set").into(),
//         "pool_size" => 10.into(),
//         "timeout" => 5.into(),
//     };

//     let figment = rocket::Config::figment()
//         .merge(("databases", map!["pgcon" => db]));

//     rocket::custom(figment).mount("/", routes![index, list_items, create_items, delete_items, edit_items]).attach(PgDbConn::fairing())
// }