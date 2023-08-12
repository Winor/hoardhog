use actix_web::web;

pub mod dbq;
pub mod list;
pub mod delete;
pub mod create;
pub mod update;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .service(list::items)
            .service(create::item)
            .service(delete::item)
            .service(update::item)
    );
}