use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CarSchema {
    pub make: String,
    pub model: String,
    pub spotted: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateCarSchema {
    pub make: Option<String>,
    pub model: Option<String>,
    pub spotted: Option<i32>,
}
