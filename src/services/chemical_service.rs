use rusqlite::{Connection, params};
use crate::models::chemical::ChemicalRecord;

pub fn get_all(conn: &Connection) -> Vec<ChemicalRecord> {
    let mut stmt = conn.prepare(
        "SELECT id, planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at FROM chemicals ORDER BY application_date DESC"
    ).unwrap();
    stmt.query_map([], |row| {
        Ok(ChemicalRecord {
            id: row.get(0)?,
            planting_id: row.get(1)?,
            chem_type: row.get(2)?,
            name: row.get(3)?,
            dosage: row.get(4)?,
            unit: row.get(5)?,
            application_date: row.get(6)?,
            operator: row.get(7)?,
            notes: row.get(8)?,
            created_by: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect()
}

pub fn find_by_id(conn: &Connection, id: i64) -> Option<ChemicalRecord> {
    let mut stmt = conn.prepare(
        "SELECT id, planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at FROM chemicals WHERE id = ?1"
    ).ok()?;
    stmt.query_row(params![id], |row| {
        Ok(ChemicalRecord {
            id: row.get(0)?,
            planting_id: row.get(1)?,
            chem_type: row.get(2)?,
            name: row.get(3)?,
            dosage: row.get(4)?,
            unit: row.get(5)?,
            application_date: row.get(6)?,
            operator: row.get(7)?,
            notes: row.get(8)?,
            created_by: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).ok()
}

pub fn find_by_planting_id(conn: &Connection, planting_id: i64) -> Vec<ChemicalRecord> {
    let mut stmt = conn.prepare(
        "SELECT id, planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at FROM chemicals WHERE planting_id = ?1 ORDER BY application_date DESC"
    ).unwrap();
    stmt.query_map(params![planting_id], |row| {
        Ok(ChemicalRecord {
            id: row.get(0)?,
            planting_id: row.get(1)?,
            chem_type: row.get(2)?,
            name: row.get(3)?,
            dosage: row.get(4)?,
            unit: row.get(5)?,
            application_date: row.get(6)?,
            operator: row.get(7)?,
            notes: row.get(8)?,
            created_by: row.get(9)?,
            created_at: row.get(10)?,
        })
    }).unwrap().filter_map(|r| r.ok()).collect()
}

pub fn create(
    conn: &Connection,
    planting_id: i64,
    chem_type: &str,
    name: &str,
    dosage: &str,
    unit: &str,
    application_date: &str,
    operator: &str,
    notes: Option<&str>,
    created_by: i64,
) -> Result<i64, String> {
    let created_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}
