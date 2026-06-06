use axum_extra::extract::cookie::CookieJar;
use crate::SharedState;
use crate::models::user::User;
use crate::services::user_service;

pub fn get_current_user(jar: &CookieJar, state: &SharedState) -> Option<User> {
    let user_id: i64 = jar.get("user_id")?.value().parse().ok()?;
    let db = state.db.lock().unwrap();
    user_service::find_by_id(&db, user_id)
}
