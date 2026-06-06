use rusqlite::{Connection, params};
use crate::models::harvest::Harvest;

pub fn get_all(conn: &Connection) -> Vec<Harvest> {
    let mut stmt = conn.prepare(
        "SELECT id, planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at FROM harvests ORDER BY harvest_date DESC"
    ).unwrap();
    stmt.query_map([], |row| {
        Ok(Harvest {
            id: row.get(0)?,
            planting_id: row.get(1)?,
            harvest_date: row.get(2)?,
            quantity: row.get(3)?,
            unit: row.get(4)?,
            quality_grade: row.get(5)?,
            buyer: row.get(6)?,
            price: row.get(7)?,
            notes: row.get(8)?,
            created_by: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect()
}

pub fn find_by_id(conn: &Connection, id: i64) -> Option<Harvest> {
    let mut stmt = conn.prepare(
        "SELECT id, planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at FROM harvests WHERE id = ?1"
    ).ok()?;
    stmt.query_row(params![id], |row| {
        Ok(Harvest {
            id: row.get(0)?,
            planting_id: row.get(1)?,
            harvest_date: row.get(2)?,
            quantity: row.get(3)?,
            unit: row.get(4)?,
            quality_grade: row.get(5)?,
            buyer: row.get(6)?,
            price: row.get(7)?,
            notes: row.get(8)?,
            created_by: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).ok()
}

pub fn find_by_planting_id(conn: &Connection, planting_id: i64) -> Vec<Harvest> {
    let mut stmt = conn.prepare(
        "SELECT id, planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at FROM harvests WHERE planting_id = ?1 ORDER BY harvest_date DESC"
    ).unwrap();
    stmt.query_map(params![planting_id], |row| {
        Ok(Harvest {
            id: row.get(0)?,
            planting_id: row.get(1)?,
            harvest_date: row.get(2)?,
            quantity: row.get(3)?,
            unit: row.get(4)?,
            quality_grade: row.get(5)?,
            buyer: row.get(6)?,
            price: row.get(7)?,
            notes: row.get(8)?,
            created_by: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect()
}

pub fn create(
    conn: &Connection,
    planting_id: i64,
    harvest_date: &str,
    quantity: f64,
    unit: &str,
    quality_grade: &str,
    buyer: Option<&str>,
    price: Option<f64>,
    notes: Option<&str>,
    created_by: i64,
) -> Result<i64, String> {
    let created_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO harvests (planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}
