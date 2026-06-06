use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::extract::{State, Form};
use axum_extra::extract::cookie::{CookieJar, Cookie};
use serde::Deserialize;
use tera::Context;
use crate::SharedState;
use crate::services::user_service;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub password: String,
    pub real_name: String,
}

pub async fn login_page(State(state): State<SharedState>) -> Html<String> {
    let ctx = Context::new();
    Html(state.tera.render("auth/login.html", &ctx).unwrap())
}

pub async fn login(
    State(state): State<SharedState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> (CookieJar, Response) {
    let db = state.db.lock().unwrap();
    let user = user_service::find_by_username(&db, &form.username);
    drop(db);

    match user {
        Some(u) if user_service::verify_password(&u, &form.password) => {
            let cookie = Cookie::build(("user_id", u.id.to_string()))
                .path("/")
                .http_only(true)
                .finish();
            (jar.add(cookie), Redirect::to("/dashboard").into_response())
        }
        _ => {
            let mut ctx = Context::new();
            ctx.insert("error", "用户名或密码错误");
            let html = state.tera.render("auth/login.html", &ctx).unwrap();
            (jar, Html(html).into_response())
        }
    }
}

pub async fn register_page(State(state): State<SharedState>) -> Html<String> {
    let ctx = Context::new();
    Html(state.tera.render("auth/register.html", &ctx).unwrap())
}

pub async fn register(
    State(state): State<SharedState>,
    jar: CookieJar,
    Form(form): Form<RegisterForm>,
) -> (CookieJar, Response) {
    let db = state.db.lock().unwrap();
    match user_service::create_user(&db, &form.username, &form.password, &form.real_name, "operator") {
        Ok(id) => {
            drop(db);
            let cookie = Cookie::build(("user_id", id.to_string()))
                .path("/")
                .http_only(true)
                .finish();
            (jar.add(cookie), Redirect::to("/dashboard").into_response())
        }
        Err(_) => {
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", "注册失败，用户名可能已存在");
            let html = state.tera.render("auth/register.html", &ctx).unwrap();
            (jar, Html(html).into_response())
        }
    }
}

pub async fn logout(jar: CookieJar) -> (CookieJar, Redirect) {
    let cookie = Cookie::build(("user_id", ""))
        .path("/")
        .finish();
    (jar.remove(cookie), Redirect::to("/login"))
}
