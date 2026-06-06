use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Harvest {
    pub id: i64,
    pub planting_id: i64,
    pub harvest_date: String,
    pub quantity: f64,
    pub unit: String,
    pub quality_grade: String,
    pub buyer: Option<String>,
    pub price: Option<f64>,
    pub notes: Option<String>,
    pub created_by: i64,
    pub created_at: String,
}
