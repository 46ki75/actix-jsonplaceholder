use crate::models::user::User;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::ok_response_builder::OkResponseBuilder;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_users));
}

async fn get_users(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let client = reqwest::Client::new();

    let res = client
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let body = res
        .text()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let users: Vec<User> =
        serde_json::from_str(&body).map_err(actix_web::error::ErrorInternalServerError)?;

    let connection_info = req.connection_info();
    let scheme = connection_info.scheme();
    let host = connection_info.host();
    let full_url = format!("{}://{}{}", scheme, host, req.uri());
    let data = OkResponseBuilder::new()
        .data(users)
        .self_link(full_url)
        .build();

    Ok(HttpResponse::Ok().json(data))
}
