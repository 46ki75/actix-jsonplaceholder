mod mock_controller;
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn init_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("", web::get().to(get_v1))
            .service(web::scope("/mock").configure(mock_controller::config)),
    );
}

async fn get_v1() -> HttpResponse {
    let data: serde_json::Value = json!({
        "message": "Hello, v1!"
    });
    HttpResponse::Ok().json(data)
}
