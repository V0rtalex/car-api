use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{AppState, handlers::{car_list_handler, create_car_handler, delete_car_handler, get_car_handler, update_car_handler}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/cars", get(car_list_handler))
        .route("/api/cars", post(create_car_handler))
        .route(
            "/api/cars/{id}", 
            get(get_car_handler)
                .delete(delete_car_handler)
                .patch(update_car_handler)
        )
        .with_state(app_state)
}
