use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Planting {
    pub id: i64,
    pub crop_name: String,
    pub variety: String,
    pub area: f64,
    pub planting_date: String,
    pub expected_harvest_date: Option<String>,
    pub status: String,
    pub base_id: String,
    pub created_by: i64,
    pub created_at: String,
}
