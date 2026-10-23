use crate::schema::CarSchema;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde::ser::SerializeStruct;

pub struct CarSuccess<Data = ()> {
    pub message: String,
    pub data: Data,
}

impl<Data> Serialize for CarSuccess<Data>
where
    Data: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("CarSuccess", 2)?;
        s.serialize_field("status", "success")?;
        s.serialize_field("message", &self.message)?;
        s.serialize_field("data", &self.data)?;
        s.end()
    }
}

impl<Data> TryFrom<CarSuccess<Data>> for serde_json::Value
where
    Data: Serialize,
{
    type Error = serde_json::Error;
    fn try_from(car_success: CarSuccess<Data>) -> Result<Self, Self::Error> {
        serde_json::to_value(&car_success)
    }
}

impl<Data> IntoResponse for CarSuccess<Data> 
where 
    Data: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match serde_json::Value::try_from(self) {
            Ok(json_value) => Json(json_value).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to convert JSON").into_response(),
        }
    }
}

// enum for all errors
pub enum CarError {
    CarWithIdNotFound(uuid::Uuid),
    CarAlreadyExists(CarSchema),
    UnknownError,
}

// convert error to (StatusCode, ErrorBody)
impl From<CarError> for (StatusCode, ErrorBody) {
    fn from(car_error: CarError) -> Self {
        match car_error {
            CarError::CarWithIdNotFound(id) => (
                StatusCode::NOT_FOUND,
                ErrorBody {
                    error: "cars.car_not_found".to_string(),
                    message: format!("car with id {} not found", id),
                },
            ),
            CarError::CarAlreadyExists(car) => (
                StatusCode::CONFLICT,
                ErrorBody {
                    error: "cars.car_already_exists".to_string(),
                    message: format!("car {} {} already exists in database", car.make, car.model),
                },
            ),
            CarError::UnknownError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorBody {
                    error: "unknown_error".to_string(),
                    message: "an unknowed error occured".to_string(),
                },
            ),
        }
    }
}

// impl IntoResponse for futher processing by axum
impl IntoResponse for CarError {
    fn into_response(self) -> axum::response::Response {
        <(StatusCode, ErrorBody)>::from(self).into_response()
    }
}

// easy to use error body
pub struct ErrorBody {
    pub error: String,
    pub message: String,
}

// convert to json
impl From<ErrorBody> for serde_json::Value {
    fn from(error_body: ErrorBody) -> Self {
        serde_json::json!({
            "status": "error",
            "error": error_body.error,
            "message": error_body.message,
        })
    }
}

// impl IntoResponse for further processing by axum
impl IntoResponse for ErrorBody {
    fn into_response(self) -> axum::response::Response {
        // convert self into serde_json::Value (via from ^) -> axum::Json <- impl IntoResponse
        Json(serde_json::Value::from(self)).into_response()
    }
}
