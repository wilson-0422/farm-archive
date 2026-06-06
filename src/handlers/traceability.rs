use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Path, Query};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use tera::Context;
use crate::SharedState;
use crate::middleware::auth;
use crate::services::{traceability_service, planting_service, harvest_service};

#[derive(Deserialize)]
pub struct TraceQuery {
    pub code: Option<String>,
}

pub async fn query_page(
    State(state): State<SharedState>,
    jar: CookieJar,
    Query(query): Query<TraceQuery>,
) -> Response {
    let user = auth::get_current_user(&jar, &state);

    let mut ctx = Context::new();
    if let Some(u) = &user {
        ctx.insert("user", u);
    }

    if let Some(code) = &query.code {
        let db = state.db.lock().unwrap();
        let record = traceability_service::find_by_code(&db, code);
        if let Some(r) = &record {
            let planting = planting_service::find_by_id(&db, r.planting_id);
            let harvest = harvest_service::find_by_id(&db, r.harvest_id);
            ctx.insert("record", r);
            ctx.insert("planting", &planting);
            ctx.insert("harvest", &harvest);
        } else {
            ctx.insert("not_found", &true);
        }
        drop(db);
        ctx.insert("query_code", code);
    }

    Html(state.tera.render("traceability/query.html", &ctx).unwrap()).into_response()
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
    let records = traceability_service::get_all(&db);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("records", &records);
    Html(state.tera.render("traceability/list.html", &ctx).unwrap()).into_response()
}

pub async fn detail(
    State(state): State<SharedState>,
    jar: CookieJar,
    Path(code): Path<String>,
) -> Response {
    let user = auth::get_current_user(&jar, &state);

    let db = state.db.lock().unwrap();
    let record = match traceability_service::find_by_code(&db, &code) {
        Some(r) => r,
        None => return Redirect::to("/traceability").into_response(),
    };
    let planting = planting_service::find_by_id(&db, record.planting_id);
    let harvest = harvest_service::find_by_id(&db, record.harvest_id);
    drop(db);

    let mut ctx = Context::new();
    if let Some(u) = &user {
        ctx.insert("user", u);
    }
    ctx.insert("record", &record);
    ctx.insert("planting", &planting);
    ctx.insert("harvest", &harvest);
    Html(state.tera.render("traceability/detail.html", &ctx).unwrap()).into_response()
}
