use rusqlite::{Connection, params};
use crate::models::planting::Planting;

pub fn get_all(conn: &Connection) -> Vec<Planting> {
    let mut stmt = conn.prepare(
        "SELECT id, crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at FROM plantings ORDER BY created_at DESC"
    ).unwrap();
    stmt.query_map([], |row| {
        Ok(Planting {
            id: row.get(0)?,
            crop_name: row.get(1)?,
            variety: row.get(2)?,
            area: row.get(3)?,
            planting_date: row.get(4)?,
            expected_harvest_date: row.get(5)?,
            status: row.get(6)?,
            base_id: row.get(7)?,
            created_by: row.get(8)?,
            created_at: row.get(9)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect()
}

pub fn find_by_id(conn: &Connection, id: i64) -> Option<Planting> {
    let mut stmt = conn.prepare(
        "SELECT id, crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at FROM plantings WHERE id = ?1"
    ).ok()?;
    stmt.query_row(params![id], |row| {
        Ok(Planting {
            id: row.get(0)?,
            crop_name: row.get(1)?,
            variety: row.get(2)?,
            area: row.get(3)?,
            planting_date: row.get(4)?,
            expected_harvest_date: row.get(5)?,
            status: row.get(6)?,
            base_id: row.get(7)?,
            created_by: row.get(8)?,
            created_at: row.get(9)?,
        })
    }).ok()
}

pub fn create(
    conn: &Connection,
    crop_name: &str,
    variety: &str,
    area: f64,
    planting_date: &str,
    expected_harvest_date: Option<&str>,
    status: &str,
    base_id: &str,
    created_by: i64,
) -> Result<i64, String> {
    let created_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO plantings (crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update(
    conn: &Connection,
    id: i64,
    crop_name: &str,
    variety: &str,
    area: f64,
    planting_date: &str,
    expected_harvest_date: Option<&str>,
    status: &str,
    base_id: &str,
) -> Result<(), String> {
    conn.execute(
        "UPDATE plantings SET crop_name = ?1, variety = ?2, area = ?3, planting_date = ?4, expected_harvest_date = ?5, status = ?6, base_id = ?7 WHERE id = ?8",
        params![crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
