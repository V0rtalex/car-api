use std::sync::Arc;

use crate::{
    AppState,
    handlers_responses::*,
    model::CarModel,
    schema::{CarSchema, UpdateCarSchema},
};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::postgres::PgDatabaseError;
use uuid::Uuid;

//#[axum::debug_handler]
pub async fn create_car_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CarSchema>,
) -> Result<CarSuccess<CarModel>, CarError> {
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
    .await;

    match car {
        Ok(car) => Ok(CarSuccess {
            message: "car added successfully".to_string(),
            data: car,
        }),

        Err(sqlx::Error::Database(db_err)) => match db_err.try_downcast::<PgDatabaseError>() {
            Ok(pg_err) => match pg_err.code() {
                "23505" => Err(CarError::CarAlreadyExists(body)),
                _ => Err(CarError::UnknownError),
            },
            Err(_) => Err(CarError::UnknownError),
        },
        Err(_) => Err(CarError::UnknownError),
    }
}

//#[axum::debug_handler]
pub async fn car_list_handler(
    State(data): State<Arc<AppState>>,
) -> Result<CarSuccess<Vec<CarModel>>, CarError> {
    let cars = sqlx::query_as!(CarModel, r#"SELECT * FROM cars ORDER by make"#)
        .fetch_all(&data.db)
        .await;

    match cars {
        Ok(cars) => Ok(CarSuccess {
            message: "Car list fetched successfully".to_string(),
            data: cars,
        }),

        Err(_) => Err(CarError::UnknownError),
    }
}

//#[axum::debug_handler]
pub async fn get_car_handler(
    Path(car_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<CarSuccess<CarModel>, CarError> {
    let query_result = sqlx::query_as!(CarModel, r#"SELECT * FROM cars WHERE id = $1"#, car_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(car) => Ok(CarSuccess {
            message: "car fetched successfully".to_string(),
            data: car,
        }),
        Err(sqlx::Error::RowNotFound) => Err(CarError::CarWithIdNotFound(car_id)),
        Err(_) => Err(CarError::UnknownError),
    }
}

//#[axum::debug_handler]
pub async fn delete_car_handler(
    Path(car_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<CarSuccess<CarModel>, CarError> {
    let query_result = sqlx::query_as!(
        CarModel,
        r#"DELETE FROM cars WHERE id=$1 RETURNING *"#,
        &car_id
    )
    .fetch_optional(&data.db)
    .await;

    match query_result {
        Ok(Option::None) => Err(CarError::CarWithIdNotFound(car_id)),
        Ok(Some(car)) =>  Ok(CarSuccess{
            message: "car deleted successfully".to_string(),
            data: car,
        }),

        Err(_) => Err(CarError::UnknownError),
    }
}

//#[axum::debug_handler]
pub async fn update_car_handler(
    Path(car_id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateCarSchema>,
) -> Result<CarSuccess<CarModel>, CarError> {
    let query_result = sqlx::query_as!(CarModel, r#"SELECT * FROM cars WHERE id=$1"#, &car_id)
        .fetch_one(&data.db)
        .await;

    let car = match query_result {
        Ok(car) => car,
        Err(sqlx::Error::RowNotFound) => {
            return Err(CarError::CarWithIdNotFound(car_id));
        }
        Err(_) => {
            return Err(CarError::UnknownError);
        }
    };

    let new_make = body.make.as_ref().unwrap_or(&car.make);
    let new_model = body.model.as_ref().unwrap_or(&car.model);
    let new_spotted = body.spotted.unwrap_or(car.spotted);

    let updated_car = sqlx::query_as!(
        CarModel,
        r#"UPDATE cars SET make = $1, model = $2, spotted = $3 WHERE id=$4 RETURNING *"#,
        &new_make,
    
        &new_model,
        &new_spotted,
        &car_id
    )
    .fetch_one(&data.db)
    .await;

    
    match updated_car {
        Ok(updated_car) => Ok(CarSuccess{
            message: "car successfully updated".to_string(),
            data: updated_car,
        }),
        Err(_) => Err(CarError::UnknownError),
    }
}
