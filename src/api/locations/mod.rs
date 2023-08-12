use actix_web::web;

pub mod dbq;
pub mod list;
pub mod delete;
pub mod create;
pub mod update;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/locations")
            .service(list::items)
            .service(create::create)
            .service(delete::delete)
            .service(update::update)
    );
}