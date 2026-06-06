use rusqlite::{Connection, params};
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::models::user::User;

pub fn find_by_username(conn: &Connection, username: &str) -> Option<User> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, real_name, role, created_at FROM users WHERE username = ?1"
    ).ok()?;
    stmt.query_row(params![username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            real_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).ok()
}

pub fn find_by_id(conn: &Connection, id: i64) -> Option<User> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, real_name, role, created_at FROM users WHERE id = ?1"
    ).ok()?;
    stmt.query_row(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            real_name: row.get(3)?,
            role: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).ok()
}

pub fn create_user(conn: &Connection, username: &str, password: &str, real_name: &str, role: &str) -> Result<i64, String> {
    let password_hash = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    let created_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO users (username, password_hash, real_name, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![username, password_hash, real_name, role, created_at],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn verify_password(user: &User, password: &str) -> bool {
    verify(password, &user.password_hash).unwrap_or(false)
}
