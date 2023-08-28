//! Grocery controller
//!
//! All operations to doing CRUD Operations comes in this module

use actix_web::{get, HttpResponse, Responder};

/// Get list of registered shoppings
#[get("/")]
pub async fn shopping_list() -> impl Responder {
    // TODO: shopping list (json result)
    HttpResponse::Ok().body("Shopping List")
}
