use actix_web::web;

pub mod list;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/app")
            .service(list::list)
    );
}