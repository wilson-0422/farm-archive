use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Form, Path};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use tera::Context;
use crate::SharedState;
use crate::middleware::auth;
use crate::services::{planting_service, chemical_service, harvest_service};

#[derive(Deserialize)]
pub struct PlantingForm {
    pub crop_name: String,
    pub variety: String,
    pub area: String,
    pub planting_date: String,
    pub expected_harvest_date: String,
    pub status: String,
    pub base_id: String,
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
    let plantings = planting_service::get_all(&db);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("plantings", &plantings);
    Html(state.tera.render("plantings/list.html", &ctx).unwrap()).into_response()
}

pub async fn create_page(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    Html(state.tera.render("plantings/create.html", &ctx).unwrap()).into_response()
}

pub async fn create(
    State(state): State<SharedState>,
    jar: CookieJar,
    Form(form): Form<PlantingForm>,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let area: f64 = form.area.parse().unwrap_or(0.0);
    let expected = if form.expected_harvest_date.is_empty() {
        None
    } else {
        Some(form.expected_harvest_date.as_str())
    };

    let db = state.db.lock().unwrap();
    let _ = planting_service::create(
        &db,
        &form.crop_name,
        &form.variety,
        area,
        &form.planting_date,
        expected,
        &form.status,
        &form.base_id,
        user.id,
    );
    drop(db);

    Redirect::to("/plantings").into_response()
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
    let planting = match planting_service::find_by_id(&db, id) {
        Some(p) => p,
        None => return Redirect::to("/plantings").into_response(),
    };
    let chemicals = chemical_service::find_by_planting_id(&db, id);
    let harvests = harvest_service::find_by_planting_id(&db, id);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("planting", &planting);
    ctx.insert("chemicals", &chemicals);
    ctx.insert("harvests", &harvests);
    Html(state.tera.render("plantings/detail.html", &ctx).unwrap()).into_response()
}

pub async fn edit_page(
    State(state): State<SharedState>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let db = state.db.lock().unwrap();
    let planting = match planting_service::find_by_id(&db, id) {
        Some(p) => p,
        None => return Redirect::to("/plantings").into_response(),
    };
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("planting", &planting);
    Html(state.tera.render("plantings/edit.html", &ctx).unwrap()).into_response()
}

pub async fn update(
    State(state): State<SharedState>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Form(form): Form<PlantingForm>,
) -> Response {
    let _user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let area: f64 = form.area.parse().unwrap_or(0.0);
    let expected = if form.expected_harvest_date.is_empty() {
        None
    } else {
        Some(form.expected_harvest_date.as_str())
    };

    let db = state.db.lock().unwrap();
    let _ = planting_service::update(
        &db,
        id,
        &form.crop_name,
        &form.variety,
        area,
        &form.planting_date,
        expected,
        &form.status,
        &form.base_id,
    );
    drop(db);

    Redirect::to("/plantings").into_response()
}
