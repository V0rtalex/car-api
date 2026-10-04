use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CarSchema {
    pub make: String,
    pub model: String,
    pub spotted: i32,
}
