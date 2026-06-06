use rusqlite::Connection;

pub fn init_database() -> Result<Connection, String> {
    let conn = Connection::open("farm_archive.db").map_err(|e| e.to_string())?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            real_name TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'operator',
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS plantings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            crop_name TEXT NOT NULL,
            variety TEXT NOT NULL,
            area REAL NOT NULL,
            planting_date TEXT NOT NULL,
            expected_harvest_date TEXT,
            status TEXT NOT NULL DEFAULT 'growing',
            base_id TEXT NOT NULL,
            created_by INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (created_by) REFERENCES users(id)
        );
        CREATE TABLE IF NOT EXISTS chemicals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            planting_id INTEGER NOT NULL,
            chem_type TEXT NOT NULL,
            name TEXT NOT NULL,
            dosage TEXT NOT NULL,
            unit TEXT NOT NULL,
            application_date TEXT NOT NULL,
            operator TEXT NOT NULL,
            notes TEXT,
            created_by INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (planting_id) REFERENCES plantings(id),
            FOREIGN KEY (created_by) REFERENCES users(id)
        );
        CREATE TABLE IF NOT EXISTS harvests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            planting_id INTEGER NOT NULL,
            harvest_date TEXT NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT NOT NULL,
            quality_grade TEXT NOT NULL,
            buyer TEXT,
            price REAL,
            notes TEXT,
            created_by INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (planting_id) REFERENCES plantings(id),
            FOREIGN KEY (created_by) REFERENCES users(id)
        );
        CREATE TABLE IF NOT EXISTS traceability (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            planting_id INTEGER NOT NULL,
            harvest_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            origin TEXT NOT NULL,
            harvest_date TEXT NOT NULL,
            inspection_result TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (planting_id) REFERENCES plantings(id),
            FOREIGN KEY (harvest_id) REFERENCES harvests(id)
        );",
    )
    .map_err(|e| e.to_string())?;
    Ok(conn)
}
