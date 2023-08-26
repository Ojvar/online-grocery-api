use crate::utils::ping::ping;
use actix_web::{web, App, HttpServer};

// Listen function
// TODO: use _ip instead of 8080
pub async fn listen(host: &str, _ip: usize) -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ping", web::get().to(ping)))
        .bind((host, 8080))?
        .run()
        .await
}
