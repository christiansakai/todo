use std::env;
use std::collections::HashMap;

use actix::{System};
use actix_web::{
    middleware,
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
    env::set_var("RUST_LOG",  "actix_web=info");
    env_logger::init();

    let sys = System::new("todo");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .handler("/static", fs::StaticFiles::new("./static/").unwrap())
            .resource("/", |r| r.method(http::Method::GET).with(index)) 
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");

    let _ = sys.run();
}
