use std::env;

use actix::{System};
use actix_web::{middleware, server};
use actix_web::{App, HttpRequest};

fn main() {
    env::set_var("RUST_LOG",  "actix_web=info");
    env_logger::init();

    let sys = System::new("todo");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(index)) 
            .resource("/", |r| r.f(index)) 
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");

    let _ = sys.run();
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}
