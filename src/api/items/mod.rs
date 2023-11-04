use actix_web::web;
pub mod list;
pub mod delete;
pub mod create;
pub mod update;
pub mod find;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .service(list::list)
            .service(create::item)
            .service(delete::item)
            .service(update::item)
            .service(find::item)
    );
}