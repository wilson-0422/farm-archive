use rusqlite::{Connection, params};
use bcrypt::{hash, DEFAULT_COST};

pub fn seed_data(conn: &Connection) -> Result<(), String> {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    if count > 0 {
        return Ok(());
    }

    let admin_hash = hash("admin123", DEFAULT_COST).map_err(|e| e.to_string())?;
    let zhang_hash = hash("zhang123", DEFAULT_COST).map_err(|e| e.to_string())?;
    let li_hash = hash("li123", DEFAULT_COST).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO users (username, password_hash, real_name, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["admin", admin_hash, "管理员", "admin", "2025-01-01 08:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO users (username, password_hash, real_name, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["zhangwei", zhang_hash, "张伟", "operator", "2025-01-15 09:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO users (username, password_hash, real_name, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["lina", li_hash, "李娜", "operator", "2025-02-01 10:00:00"],
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO plantings (crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params!["水稻", "杂交粳稻", 50.0, "2025-03-15", "2025-09-15", "growing", "BASE-001", 1, "2025-03-15 08:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO plantings (crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params!["番茄", "大红番茄", 20.0, "2025-04-01", "2025-07-01", "growing", "BASE-001", 2, "2025-04-01 09:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO plantings (crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params!["黄瓜", "密刺黄瓜", 15.0, "2025-04-10", "2025-06-20", "harvested", "BASE-002", 2, "2025-04-10 10:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO plantings (crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params!["小麦", "冬小麦", 100.0, "2024-10-01", "2025-06-01", "harvested", "BASE-003", 3, "2024-10-01 08:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO plantings (crop_name, variety, area, planting_date, expected_harvest_date, status, base_id, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params!["玉米", "甜玉米", 30.0, "2025-04-20", "2025-08-20", "growing", "BASE-003", 3, "2025-04-20 07:30:00"],
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![1, "fertilizer", "复合肥", "50", "kg/亩", "2025-03-20", "张伟", "基肥施用", 2, "2025-03-20 09:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![1, "pesticide", "草甘膦", "200", "ml/亩", "2025-04-05", "张伟", "杂草防治", 2, "2025-04-05 10:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![2, "fertilizer", "尿素", "30", "kg/亩", "2025-04-15", "李娜", "追肥", 3, "2025-04-15 08:30:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![2, "pesticide", "吡虫啉", "50", "g/亩", "2025-05-01", "李娜", "蚜虫防治", 3, "2025-05-01 09:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![4, "fertilizer", "磷酸二铵", "40", "kg/亩", "2025-03-10", "张伟", "返青肥", 2, "2025-03-10 07:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO chemicals (planting_id, chem_type, name, dosage, unit, application_date, operator, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![5, "fertilizer", "复合肥", "45", "kg/亩", "2025-04-25", "李娜", "底肥施用", 3, "2025-04-25 08:00:00"],
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO harvests (planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![3, "2025-06-20", 45000.0, "kg", "一等", "永辉超市", 2.5, "黄瓜丰收", 2, "2025-06-20 16:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO harvests (planting_id, harvest_date, quantity, unit, quality_grade, buyer, price, notes, created_by, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![4, "2025-06-01", 500000.0, "kg", "优等", "中粮集团", 2.8, "冬小麦丰收", 3, "2025-06-01 15:00:00"],
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO traceability (code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params!["TR-2025-001", 3, 1, "密刺黄瓜", "绿源种养基地A区", "2025-06-20", "合格", "2025-06-20 17:00:00"],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO traceability (code, planting_id, harvest_id, product_name, origin, harvest_date, inspection_result, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params!["TR-2025-002", 4, 2, "冬小麦", "绿源种养基地C区", "2025-06-01", "合格", "2025-06-01 16:00:00"],
    ).map_err(|e| e.to_string())?;

    Ok(())
}
