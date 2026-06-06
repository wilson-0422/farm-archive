use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Traceability {
    pub id: i64,
    pub code: String,
    pub planting_id: i64,
    pub harvest_id: i64,
    pub product_name: String,
    pub origin: String,
    pub harvest_date: String,
    pub inspection_result: String,
    pub created_at: String,
}
