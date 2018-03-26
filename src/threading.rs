// actix-web the server framework we are using.
extern crate actix;
extern crate actix_web;
use actix::*;
use actix_web::*;

use std::thread;
use std::sync::mpsc;

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
    let (tx, rx) = mpsc::channel();

    // Start stuff in a new thread.
    thread::spawn(move || {
        // HttpServer is an actix actor.
        // It has to be initialized within properly configured actix system.
        // Here we initialize such a system.
        let sys = actix::System::new("rugi");

        println!("Trying to start on localhost:{}.", PORT.to_string());
        // run a new server on some port
        let addr = HttpServer::new(
            // make a new application
            // apparently here a lambda expression is needed
            // This is what the guide calls "Application factory",
            // because you get a new Application object when you call this procedure.
            || Application::new()
                .prefix("/")
                .resource("/index", |req| req.f(index)))
            .threads(4) // <- Start 4 workers
            .bind(format!("localhost:{}", PORT)).expect(&format!("Could not bind to port {}", PORT))
            .shutdown_timeout(60)
            .start();  // <- use start_tls() for TLS

        // send back our address so that the main thread can communicate with us
        let _ = tx.send(addr);
        // run the actix system.
        // This will start our server.
        let _ = sys.run();
    });

    // Receive the address of the started server from the channel.
    let addr = rx.recv().unwrap();
    // Send a stop message to the server's address.
    let _ = addr.send(server::StopServer{graceful:true});
}
