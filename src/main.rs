mod config;
mod handlers;
mod middleware;
mod models;
mod services;

use axum::Router;
use axum::routing::{get, post};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tera::Tera;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub tera: Tera,
}

pub type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = config::database::init_database().expect("数据库初始化失败");
    config::seed::seed_data(&db).expect("种子数据初始化失败");

    let tera = Tera::new("templates/**/*").expect("模板初始化失败");

    let state = Arc::new(AppState {
        db: Mutex::new(db),
        tera,
    });

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/register", get(handlers::auth::register_page).post(handlers::auth::register))
        .route("/logout", post(handlers::auth::logout))
        .route("/dashboard", get(handlers::home::dashboard))
        .route("/plantings", get(handlers::planting::list).post(handlers::planting::create))
        .route("/plantings/new", get(handlers::planting::create_page))
        .route("/plantings/{id}", get(handlers::planting::detail).post(handlers::planting::update))
        .route("/plantings/{id}/edit", get(handlers::planting::edit_page))
        .route("/chemicals", get(handlers::chemical::list).post(handlers::chemical::create))
        .route("/chemicals/new", get(handlers::chemical::create_page))
        .route("/chemicals/{id}", get(handlers::chemical::detail))
        .route("/harvests", get(handlers::harvest::list).post(handlers::harvest::create))
        .route("/harvests/new", get(handlers::harvest::create_page))
        .route("/harvests/{id}", get(handlers::harvest::detail))
        .route("/traceability", get(handlers::traceability::query_page))
        .route("/traceability/list", get(handlers::traceability::list))
        .route("/traceability/{code}", get(handlers::traceability::detail))
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("服务器启动: http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
