use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::models::ok_response_builder::OkResponseBuilder;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_mock))
        .route("/{slug}", web::get().to(get_route_path));
}

async fn get_mock(req: HttpRequest) -> HttpResponse {
    let connection_info = req.connection_info();
    let scheme = connection_info.scheme(); // "http" or "https"
    let host = connection_info.host(); // "hostname:port"
    let full_url = format!("{}://{}{}", scheme, host, req.uri());

    let data = OkResponseBuilder::new()
        .push_data(json!({"id":1,"status":"public"}))
        .push_data(json!({"id":2,"status":"protected"}))
        .self_link(full_url)
        .build();

    HttpResponse::Ok().json(data)
}

async fn get_route_path(req: HttpRequest, slug: web::Path<String>) -> HttpResponse {
    let connection_info = req.connection_info();
    let scheme = connection_info.scheme(); // "http" or "https"
    let host = connection_info.host(); // "hostname:port"
    let full_url = format!("{}://{}{}", scheme, host, req.uri());

    let data = OkResponseBuilder::new()
        .push_data(json!({"id":1,"status":"public","slug":*slug}))
        .self_link(full_url)
        .build();

    HttpResponse::Ok().json(data)
}
