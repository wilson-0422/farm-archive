use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Form, Path};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use tera::Context;
use crate::SharedState;
use crate::middleware::auth;
use crate::services::{chemical_service, planting_service};

#[derive(Deserialize)]
pub struct ChemicalForm {
    pub planting_id: String,
    pub chem_type: String,
    pub name: String,
    pub dosage: String,
    pub unit: String,
    pub application_date: String,
    pub operator: String,
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
    let chemicals = chemical_service::get_all(&db);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("chemicals", &chemicals);
    Html(state.tera.render("chemicals/list.html", &ctx).unwrap()).into_response()
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
    Html(state.tera.render("chemicals/create.html", &ctx).unwrap()).into_response()
}

pub async fn create(
    State(state): State<SharedState>,
    jar: CookieJar,
    Form(form): Form<ChemicalForm>,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let planting_id: i64 = form.planting_id.parse().unwrap_or(0);
    let notes = if form.notes.is_empty() {
        None
    } else {
        Some(form.notes.as_str())
    };

    let db = state.db.lock().unwrap();
    let _ = chemical_service::create(
        &db,
        planting_id,
        &form.chem_type,
        &form.name,
        &form.dosage,
        &form.unit,
        &form.application_date,
        &form.operator,
        notes,
        user.id,
    );
    drop(db);

    Redirect::to("/chemicals").into_response()
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
    let chemical = match chemical_service::find_by_id(&db, id) {
        Some(c) => c,
        None => return Redirect::to("/chemicals").into_response(),
    };
    let planting = planting_service::find_by_id(&db, chemical.planting_id);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("chemical", &chemical);
    ctx.insert("planting", &planting);
    Html(state.tera.render("chemicals/detail.html", &ctx).unwrap()).into_response()
}
