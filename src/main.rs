use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
mod controllers;
mod models;
use models::err_response_builder::ErrResponseBuilder;
use models::ok_response_builder::OkResponseBuilder;

async fn index() -> HttpResponse {
    let data = OkResponseBuilder::new()
        .data(vec![json!({"message":"Hello, Actix!"})])
        .build();
    HttpResponse::Ok().json(data)
}

async fn get_error() -> HttpResponse {
    let data = ErrResponseBuilder::new(404).build();
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("http://localhost:18080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/error", web::get().to(get_error))
            // .configure(controllers::v1::user_controller::config)
            .configure(controllers::init_app_config)
    })
    .bind("127.0.0.1:18080")?
    .workers(4)
    .run()
    .await
}
