mod utils;

use utils::rest::core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::listen("127.0.0.1", 8080).await
}
