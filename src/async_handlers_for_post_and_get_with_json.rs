// actix-web the server framework we are using.
extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate shogi;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// use actix::*;
use actix_web::*;

// use std::thread;
// use std::sync::mpsc;

mod model;
use model::*;
use futures::Future;
use futures::future::result;

// from shogi-rs
use shogi::bitboard::Factory as BBFactory;

// from my own shogi functions
// those are thin wrappers around the ones provided by shogi-rs
mod shogi_functions;

// Globals are declared outside all other scopes.
static PORT: u32 = 8000;

// A request handler is a function that accepts a HttpRequest instance as its only parameter
// and returns a type that can be converted into HttpResponse:
#[allow(dead_code)]
fn index(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // asynchronously answer requests
    result(HttpResponse::Ok()
           .content_type("text/html")
           .body(format!("Hello!"))
           .map_err(|err| err.into()))
        .responder()
}

fn try_move_with_position_get(request: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    println!(".headers().keys() of HttpRequest: {:?}", request.headers().keys());
    println!(".headers().values() of HttpRequest: {:?}", request.headers().values());
    match request.query().get("json") {
        Some(json_str) => {
            println!("json_str: {}", json_str);
            let val: TryMoveInPositionRequest = serde_json::from_str(json_str)
                .expect("could not deserialize query parameter");
            println!("val: {}", val);
            result(HttpResponse::Ok()
                   .content_type("text/html")
                   .body(format!("Hello!"))
                   .map_err(|err| err.into()))
                .responder()
        },
        None => result(
            HttpResponse::Ok()
                .content_type("text/html")
                .body(format!("No JSON in query parameters detected!"))
                .map_err(|err| err.into())
        ).responder()
    }
}

fn try_move_with_position(request: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    request.json::<TryMoveInPositionRequest>().from_err()
        .and_then(|val| {
            println!("model: {}", val);
            Ok(httpcodes::HttpOk.build().json(val)?)
        }).responder()
}

fn main() {
    // Shogi-rs initialization
    // for the shogi library to work properly initialize the bit board factory:
    BBFactory::init();

    // HttpServer is an actix actor.
    // It has to be initialized within properly configured actix system.
    // Here we initialize such a system.
    let sys = actix::System::new("rugi");

    println!("Trying to run on localhost:{}.", PORT.to_string());

    // run a new server on some port
    let _addr = HttpServer::new(
        // Make a new application.
        // Apparently here a lambda expression is needed.
        // This is what the guide calls "Application factory",
        // because you get a new Application object when you call this procedure.

        // For `Route`s we can use:
        // (1) `.f(<function here>)` for setting a handler function (that is why it is called f) or
        // (2) `.route().a(<function here>)` for setting an asynchronous handler function
        // .f(<function here>) can also be used on the `Request` itself,
        // which is a shortcut for `.route.f(<function here>)`.
        // (3) others (see docs of actix-web, they are explained there a little)

        // The routes can be filtered by predicates.
        // One common example is to filter for GET requests.
        // This can be done by adding `filter(pred::Get())` after `.route()`.
        || Application::new()
            .prefix("/")
            .resource("/try_move_with_position",
                      |req| req
                      .method(Method::POST)
                      .a(try_move_with_position))
            .resource("/try_move_with_position_get",
                      |req| req
                      .method(Method::GET)
                      .a(try_move_with_position_get)))
        .threads(4) // <- Start 4 workers
        .bind(format!("localhost:{}", PORT)).expect(&format!("Could not bind to port {}", PORT))
        .shutdown_timeout(60)
        .start();  // <- use start_tls() for TLS

    // run the actix system.
    // This will start our server.
    let _ = sys.run();
}
