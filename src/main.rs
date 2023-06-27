mod controller;
mod model;
mod view;

use controller::SomeController;
use std::collections::HashMap;

use crate::model::SomeModel;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let app: Router = app();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000)); // <- localhost:3000
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    let hash_map = HashMap::<String, SomeModel>::new(); // <- adjust to your needs
    let thread_safe_hash_map = Arc::new(Mutex::new(hash_map));
    let database = thread_safe_hash_map;

    let app = Router::new()
        .route("/some-route/:id", get(SomeController::get_something))
        .with_state(database); // <- gives your server (handlers) access to database
    app
}
