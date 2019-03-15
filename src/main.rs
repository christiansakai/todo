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
#[template(path = "todo/list.html")]
struct TodoList {
    todos: Vec<TodoShow>,
} 

#[derive(Template)]
#[template(path = "todo/create.html")]
struct TodoCreate;

#[derive(Template)]
#[template(path = "todo/edit.html")]
struct TodoEdit;

#[derive(Template)]
#[template(path = "todo/show.html")]
struct TodoShow {
    id: i32,
    title: String,
    description: String,
}

// #[derive(Template)]
// #[template(path = "user.html")]
// struct UserTemplate<'a> {
//     name: &'a str,
//     text: &'a str,
// }

// fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
//     let s = if let Some(name) = query.get("name") {
//         UserTemplate {
//             name: name,
//             text: "Welcome!",
//         }.render()
//             .unwrap()
//     } else{
//         Index.render().unwrap()
//     };

//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn list(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = TodoList {
        todos: vec![
            TodoShow {
                id: 1,
                title: "Hello World".to_string(),
                description: "Hello worlddd duuuudee".to_string(),
            },
            TodoShow {
                id: 2,
                title: "Hello World 2".to_string(),
                description: "Hello worlddd duuuudee 2".to_string(),
            },
        ],
    }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn create(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn edit(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = TodoEdit.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn show(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = TodoShow {
        id: 1,
        title: "Hello World".to_string(),
        description: "Hello worlddd duuuudee".to_string(),
    }.render().unwrap();
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
            .route("/", http::Method::GET, index) 
            .route("/todo", http::Method::GET, list) 
            .route("/todo/create", http::Method::GET, create) 
            .route("/todo/{id}", http::Method::GET, show) 
            .route("/todo/{id}/edit", http::Method::GET, edit) 
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    let _ = system.run();
}
