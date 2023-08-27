use crate::utils::ping::ping;
use actix_web::{web, App, HttpServer};

use super::static_routes;

// Listen function
// TODO: use _ip instead of 8080
pub async fn listen(host: &str, port: usize) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/{filename:.+}", web::get().to(static_routes::index))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
