#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

// use rocket_sync_db_pools::{database, diesel};
// mod routes;
mod models;
mod db;
mod schema;
use self::db::PgDbConn;
use self::models::*;
use rocket::figment::{value::{Map, Value}, util::map};
use dotenvy::dotenv;
use std::env;

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    let db: Map<_, Value> = map! {
        "url" => env::var("DATABASE_URL").expect("DATABASE_URL must be set").into(),
        "pool_size" => 10.into(),
        "timeout" => 5.into(),
    };

    let figment = rocket::Config::figment()
        .merge(("databases", map!["pgcon" => db]));

    rocket::custom(figment).mount("/", routes![index, list_items, create_items, delete_items, edit_items]).attach(PgDbConn::fairing())
}