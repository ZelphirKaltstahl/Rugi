// actix-web the server framework we are using.
extern crate actix;
extern crate actix_web;
use actix::*;
use actix_web::*;

use std::thread;
use std::sync::mpsc;
use std::cell::Cell;

mod model;
use model::*;

// Globals are declared outside all other scopes.
static PORT: u32 = 8000;

// A request handler is a function that accepts a HttpRequest instance as its only parameter
// and returns a type that can be converted into HttpResponse:
fn index(_req: HttpRequest) -> String {
    String::from("Hello World!")
}


fn main() {
    // HttpServer is an actix actor.
    // It has to be initialized within properly configured actix system.
    // Here we initialize such a system.
    let sys = actix::System::new("rugi");

    println!("Trying to start on localhost:{}.", PORT.to_string());
    // run a new server on some port
    HttpServer::new(
        // make a new application
        // apparently here a lambda expression is needed
        // This is what the guide calls "Application factory",
        // because you get a new Application object when you call this procedure.
        || Application::new()
            .prefix("/")
            .resource("/index", |req| req.f(index)))
        .bind(format!("localhost:{}", PORT))
        .expect(&format!("Could not bind to port {}", PORT))
        .start();  // <- use start_tls() for TLS

    // We can run multiple `HttpServer`s in the same system.
    HttpServer::new(
        || Application::new()
            .resource("/", |r| r.h(httpcodes::HttpOk)))
        .bind("127.0.0.1:59080").unwrap()
        .start();

    let _ = sys.run();

    // an application with mutable state
    // Application::with_state(AppState{counter: Cell::new(0)})
    //     .resource("/", |r| r.method(Method::GET).f(index))
    //     .finish();
}
