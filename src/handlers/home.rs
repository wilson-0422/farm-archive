use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::State;
use axum_extra::extract::cookie::CookieJar;
use tera::Context;
use crate::SharedState;
use crate::middleware::auth;
use crate::services::{planting_service, chemical_service, harvest_service, traceability_service};

pub async fn index(State(state): State<SharedState>) -> Html<String> {
    let ctx = Context::new();
    Html(state.tera.render("index.html", &ctx).unwrap())
}

pub async fn dashboard(
    State(state): State<SharedState>,
    jar: CookieJar,
) -> Response {
    let user = match auth::get_current_user(&jar, &state) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let db = state.db.lock().unwrap();
    let plantings = planting_service::get_all(&db);
    let chemicals = chemical_service::get_all(&db);
    let harvests = harvest_service::get_all(&db);
    let trace_records = traceability_service::get_all(&db);
    drop(db);

    let planting_count = plantings.len();
    let chemical_count = chemicals.len();
    let harvest_count = harvests.len();
    let trace_count = trace_records.len();
    let growing_count = plantings.iter().filter(|p| p.status == "growing").count();
    let harvested_count = plantings.iter().filter(|p| p.status == "harvested").count();

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("planting_count", &planting_count);
    ctx.insert("chemical_count", &chemical_count);
    ctx.insert("harvest_count", &harvest_count);
    ctx.insert("trace_count", &trace_count);
    ctx.insert("growing_count", &growing_count);
    ctx.insert("harvested_count", &harvested_count);
    ctx.insert("recent_plantings", &plantings.iter().take(5).cloned().collect::<Vec<_>>());
    ctx.insert("recent_harvests", &harvests.iter().take(5).cloned().collect::<Vec<_>>());
    Html(state.tera.render("dashboard/overview.html", &ctx).unwrap()).into_response()
}
