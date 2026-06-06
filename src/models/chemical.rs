use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChemicalRecord {
    pub id: i64,
    pub planting_id: i64,
    pub chem_type: String,
    pub name: String,
    pub dosage: String,
    pub unit: String,
    pub application_date: String,
    pub operator: String,
    pub notes: Option<String>,
    pub created_by: i64,
    pub created_at: String,
}
