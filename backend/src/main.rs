use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::routes::configure_routes;
use crate::models::AppState;

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Arc::new(Mutex::new(Vec::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(configure_routes)
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
