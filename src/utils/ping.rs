//! Ping controller
//!
//! It's provide the ping route to check health state for the service

use super::rest::types::AppState;
use actix_web::{get, web, HttpResponse, Responder};

/// Ping route
#[get("/ping")]
pub async fn ping(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("{} - Ping", data.app_name))
}
