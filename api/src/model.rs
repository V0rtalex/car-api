use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CarModel {
    pub id: Uuid,
    pub make: String,
    pub model: String,
    pub spotted: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
