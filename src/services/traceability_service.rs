use rusqlite::{Connection, params};
use crate::models::traceability::Traceability;

pub fn get_all(conn: &Connection) -> Vec<Traceability> {
    let mut stmt = conn.prepare(
        "SELECT id, code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at FROM traceability ORDER BY created_at DESC"
    ).unwrap();
    stmt.query_map([], |row| {
        Ok(Traceability {
            id: row.get(0)?,
            code: row.get(1)?,
            planting_id: row.get(2)?,
            harvest_id: row.get(3)?,
            product_name: row.get(4)?,
            origin: row.get(5)?,
            harvest_date: row.get(6)?,
            inspection_result: row.get(7)?,
            created_at: row.get(8)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect()
}

pub fn find_by_code(conn: &Connection, code: &str) -> Option<Traceability> {
    let mut stmt = conn.prepare(
        "SELECT id, code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at FROM traceability WHERE code = ?1"
    ).ok()?;
    stmt.query_row(params![code], |row| {
        Ok(Traceability {
            id: row.get(0)?,
            code: row.get(1)?,
            planting_id: row.get(2)?,
            harvest_id: row.get(3)?,
            product_name: row.get(4)?,
            origin: row.get(5)?,
            harvest_date: row.get(6)?,
            inspection_result: row.get(7)?,
            created_at: row.get(8)?,
        })
    }).ok()
}

pub fn find_by_id(conn: &Connection, id: i64) -> Option<Traceability> {
    let mut stmt = conn.prepare(
        "SELECT id, code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at FROM traceability WHERE id = ?1"
    ).ok()?;
    stmt.query_row(params![id], |row| {
        Ok(Traceability {
            id: row.get(0)?,
            code: row.get(1)?,
            planting_id: row.get(2)?,
            harvest_id: row.get(3)?,
            product_name: row.get(4)?,
            origin: row.get(5)?,
            harvest_date: row.get(6)?,
            inspection_result: row.get(7)?,
            created_at: row.get(8)?,
        })
    }).ok()
}

pub fn create(
    conn: &Connection,
    code: &str,
    planting_id: i64,
    harvest_id: i64,
    product_name: &str,
    origin: &str,
    harvest_date: &str,
    inspection_result: &str,
) -> Result<i64, String> {
    let created_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO traceability (code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}
