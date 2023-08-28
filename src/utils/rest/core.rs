//! Core module of REST
//!
//! It's powered by Actix crate
//! - Fast

use crate::utils::ping::ping;
use actix_web::{web, App, HttpServer};

use super::{shopping::shopping_list, static_routes, types::AppState};

#[doc = r"Listen function"]
pub async fn listen(host: &str, port: usize) -> std::io::Result<()> {
    HttpServer::new(move || {
        let app_state_data = web::Data::new(AppState {
            app_name: String::from("Online Grocery API"),
        });

        App::new()
            .app_data(app_state_data)
            .service(
                web::scope("/api")
                    .service(ping)
                    .service(web::scope("/shopping").service(shopping_list)),
            )
            .service(
                web::scope("/files").route("/{filename:.+}", web::get().to(static_routes::index)),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
