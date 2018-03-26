// actix-web the server framework we are using.
extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate serde_derive;

use actix::*;
use actix_web::*;

use std::thread;
use std::sync::mpsc;

mod model;
use model::*;
use futures::Future;
use futures::future::result;

// Globals are declared outside all other scopes.
static PORT: u32 = 8000;


// To return custom types (for example a Shogi game position)
// the trait `Responder` needs to be implemented for the type.
#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder implementation for MyObj
// With this implementation of the `Responder` it is possible to return a struct directly from a handler.
impl Responder for MyObj {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse> {
        let body = serde_json::to_string(&self).expect("could not serialize instance");

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body).expect("could not attach body and create response"))
    }
}

// A request handler is a function that accepts a HttpRequest instance as its only parameter
// and returns a type that can be converted into HttpResponse:
fn index(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    // asynchronously answer requests
    result(HttpResponse::Ok()
           .content_type("text/html")
           .body(format!("Hello!"))
           .map_err(|err| err.into()))
        .responder()
}

fn index_object(_req: HttpRequest) -> Box<Future<Item=MyObj, Error=Error>> {
    // asynchronously answer requests must return a future
    result(Ok(MyObj{name: "Zelphir Kaltstahl"})).responder()
}

fn main() {
    // let (tx, rx) = mpsc::channel();

    // Start stuff in a new thread.
    // thread::spawn(move || {
        // HttpServer is an actix actor.
        // It has to be initialized within properly configured actix system.
        // Here we initialize such a system.
        let sys = actix::System::new("rugi");

        println!("Trying to start on localhost:{}.", PORT.to_string());
        // run a new server on some port
        let _addr = HttpServer::new(
            // make a new application
            // apparently here a lambda expression is needed
            // This is what the guide calls "Application factory",
            // because you get a new Application object when you call this procedure.
            || Application::new()
                .prefix("/")
            // For resources we can use
            // `.f(<function here>)` for setting a handler function (that is why it is called f) or
            // `.route().a(<function here>)` for setting an asynchronous handler function
            // .f(<function here>) can also be used on the `Request` itself,
            // which is a shortcut for `.route.f(<function here>)`.

            // The routes can be filtered by predicates.
            // One common example is to filter for GET requests.
            // This can be done by adding `filter(pred::Get())` after `.route()`.
                .resource("/index", |req| req.route().a(index))
                .resource("/index_object", |req| req.route().a(index_object)))
        .threads(4) // <- Start 4 workers
        .bind(format!("localhost:{}", PORT)).expect(&format!("Could not bind to port {}", PORT))
        .shutdown_timeout(60)
        .start();  // <- use start_tls() for TLS

        // send back our address so that the main thread can communicate with us
        // let _ = tx.send(addr);
        // run the actix system.
        // This will start our server.
        let _ = sys.run();
    // });

    // Receive the address of the started server from the channel.
    // let addr = rx.recv().unwrap();
    // Send a stop message to the server's address.
    // let _ = addr.send(server::StopServer{graceful:true}).wait();
}
