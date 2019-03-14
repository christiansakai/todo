#[macro_use] extern crate diesel;

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
use dotenv::dotenv;
use diesel::Connection;
use diesel::pg::PgConnection;

mod schema;
mod models;

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

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    dotenv().ok();
    env_logger::init();

    // DB
    let connection = establish_connection();
    // let results = posts.filter(published.eq(true))
    //     .limit(5)
    //     .load::<Post>(&connection)
    //     .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }

    // Web
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
