use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::{create_car_handler, hello_world}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api", get(hello_world))
        .route("/api/cars", post(create_car_handler))
        .with_state(app_state)
}
