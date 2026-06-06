use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Form, Path};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use tera::Context;
use crate::SharedState;
use crate::middleware::auth;
use crate::services::{harvest_service, planting_service};

#[derive(Deserialize)]
pub struct HarvestForm {
    pub planting_id: String,
    pub harvest_date: String,
    pub quantity: String,
    pub unit: String,
    pub quality_grade: String,
    pub buyer: String,
    pub price: String,
    pub notes: String,
}

pub async fn list(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let db = state.db.lock().unwrap();
    let harvests = harvest_service::get_all(&db);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("harvests", &harvests);
    Html(state.tera.render("harvests/list.html", &ctx).unwrap()).into_response()
}

pub async fn create_page(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let db = state.db.lock().unwrap();
    let plantings = planting_service::get_all(&db);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("plantings", &plantings);
    Html(state.tera.render("harvests/create.html", &ctx).unwrap()).into_response()
}

pub async fn create(
    State(state): State<SharedState>,
    jar: CookieJar,
    Form(form): Form<HarvestForm>,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let planting_id: i64 = form.planting_id.parse().unwrap_or(0);
    let quantity: f64 = form.quantity.parse().unwrap_or(0.0);
    let buyer = if form.buyer.is_empty() {
        None
    } else {
        Some(form.buyer.as_str())
    };
    let price: Option<f64> = if form.price.is_empty() {
        None
    } else {
        form.price.parse().ok()
    };
    let notes = if form.notes.is_empty() {
        None
    } else {
        Some(form.notes.as_str())
    };

    let db = state.db.lock().unwrap();
    let _ = harvest_service::create(
        &db,
        planting_id,
        &form.harvest_date,
        quantity,
        &form.unit,
        &form.quality_grade,
        buyer,
        price,
        notes,
        user.id,
    );
    drop(db);

    Redirect::to("/harvests").into_response()
}

pub async fn detail(
    State(state): State<SharedState>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let db = state.db.lock().unwrap();
    let harvest = match harvest_service::find_by_id(&db, id) {
        Some(h) => h,
        None => return Redirect::to("/harvests").into_response(),
    };
    let planting = planting_service::find_by_id(&db, harvest.planting_id);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("harvest", &harvest);
    ctx.insert("planting", &planting);
    Html(state.tera.render("harvests/detail.html", &ctx).unwrap()).into_response()
}
