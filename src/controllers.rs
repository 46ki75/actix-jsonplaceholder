mod v1;

use actix_web::web;

pub fn init_app_config(cfg: &mut web::ServiceConfig) {
    v1::init_config(cfg);
}
