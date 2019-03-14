#[macro_use] extern crate diesel;

use std::env;
use std::collections::HashMap;

use actix::{System};
use actix_web::{
    middleware::Logger,
    server,
    http,
    fs,
    App,
    HttpRequest,
    HttpResponse,
    Query,
    Result,
};
use askama::Template;
use dotenv::dotenv;
use diesel::Connection;
use diesel::pg::PgConnection;
use log::info;

mod schema;
mod state;
mod models;

use crate::state::State;

#[derive(Template)]
#[template(path = "base.html")]
struct Base;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name: name,
            text: "Welcome!",
        }.render()
            .unwrap()
    } else {
        Index.render().unwrap()
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    let _ = dotenv().ok();
    let _ = env_logger::init();

    info!("Starting http server: 127.0.0.1:8080");

    let system = System::new("todo");
    let state = State::init();

    server::new(move || {
        App::with_state(state.clone())
            .middleware(Logger::default())
            .handler(
                "/static",
                fs::StaticFiles::new("./static/").unwrap(),
            )
            .resource("/", |r| r.method(http::Method::GET).with(index)) 
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    let _ = system.run();
}
