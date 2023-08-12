use actix_web::web;

pub mod items;
pub mod locations;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(items::init_routes)
            .configure(locations::init_routes)
    );
}