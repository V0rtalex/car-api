use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use uuid::Uuid;
use crate::{AppState, model::CarModel, schema::CarSchema};

pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello, World!"
    });
    Json(json_response)
}

pub async fn create_car_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CarSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4();
    let car = sqlx::query_as!(
        CarModel,
        r#" INSERT INTO cars (id, make, model, spotted) VALUES ($1, $2, $3, $4) RETURNING *"#,
        &id,
        &body.make,
        &body.model,
        &body.spotted
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    if let Err(err) = car {
        if err.to_string().contains("duplicate key value") {
            let error_response = serde_json::json!({
                "status": " error",
                "message": "Car already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
        return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", err)})),
        ));
    }

    let car_response = json!({
        "status": "success",
        "data": json!({
            "car": car
        })
    });

    Ok(Json(car_response))
}

pub async fn car_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cars = sqlx::query_as!(CarModel, r#"SELECT * FROM cars ORDER by make"#)
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_message = serde_json::json!({
                "status": "error",
                "message": format!("Database error { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message))
        })?;

    let json_response = serde_json::json!({
        "status": "success",
        "count": cars.len(),
        "notes": cars
    });

    Ok(Json(json_response))
}

pub async fn get_car_handler(
    Path(car_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(CarModel, r#"SELECT * FROM cars WHERE id = $1"#, car_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(car) => {
            let car_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "car": car
                })
            });
            Ok(Json(car_response))
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Car with id {} not found", car_id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
        )),
    }
}
