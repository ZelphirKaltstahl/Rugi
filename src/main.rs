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
// #[macro_use] extern crate failure;
use actix_web::{
    Application,
    AsyncResponder,
    Either,
    Error,
    HttpResponse,
    HttpRequest,
    HttpServer,
    Method,
};

mod errors;  // needed in other module
// use errors::{ShogiToShogiRsConversionError};

mod model;
use model::*;

mod conversion;
use conversion::*;

use futures::Future;
use futures::future::result;

// from shogi-rs
use shogi::bitboard::Factory as BBFactory;
use shogi::{Position};

// from my own shogi functions
// those are thin wrappers around the ones provided by shogi-rs
mod shogi_functions;
use shogi_functions::is_valid_move_in_position;

// Globals are declared outside all other scopes.
static PORT: u32 = 8000;


// The handler has to handle multiple cases.
// (1) The JSON could be missing from the query parameters.
//     --> BadRequest
// (2) The JSON could be malformed so that it cannot be deserialized into a TryMoveInPositionRequest.
//     --> BadRequest
// (3) Everything could be correct.
//     --> Box<Future<Item=HttpResponse, Error=Error>>
//
// So we create a new type for the handler to return here.
// The first alternative is the successful case.
// The second alternative is the error case, in which we still need to construct a response for the client.
type TryMoveResult = Either<Box<Future<Item=TryMoveResponse, Error=Error>>, HttpResponse>;

fn try_move_with_position(request: HttpRequest) -> TryMoveResult {
    match request.query().get("json") {
        Some(json_str) => {
            match serde_json::from_str::<TryMoveInPositionRequest>(json_str) {
                Ok(try_move_in_position_request) => {
                    let typed_resp = TryMoveResponse {
                        is_valid: validate_move(try_move_in_position_request),
                        game_status: Some(GameStatus::OnGoing)
                    };
                    // success case
                    Either::A(result(Ok(typed_resp)).responder())
                },
                Err(_some_err) => {
                    // answer with error because JSON is wrong
                    Either::B(HttpResponse::BadRequest()
                              .content_type("text/html")
                              .body("Malformed JSON, could not deserialize!")
                              .unwrap())  // <- is unwrap here safe?
                }
            }
        },
        None => {
            // answer with error because JSON is missing
            Either::B(HttpResponse::BadRequest()
                      .content_type("text/html")
                      .body("No JSON in query parameters detected!")
                      .unwrap())  // <- is unwrap here safe?
        }
    }
}

fn validate_move(try_move_in_position_request: TryMoveInPositionRequest) -> bool {
    let mut position = Position::new();
    // setting position with set_sfen can fail --> pattern matching for it later
    let set_sfen_result = position.set_sfen(&try_move_in_position_request.sfen);
    let shogi_rs_move = shogi_rs_move_from_shogi_move(try_move_in_position_request.move_in_position);

    match set_sfen_result {
        Ok(()) => match shogi_rs_move {
            Ok(shogi_rs_move) => is_valid_move_in_position(position, shogi_rs_move),
            Err(_) => false
        },
        Err(_) => false
    }
}


fn main() {
    // Shogi-rs initialization
    // for the shogi library to work properly initialize the bit board factory:
    BBFactory::init();

    // HttpServer is an actix actor.
    // It has to be initialized within properly configured actix system.
    // Here we initialize such a system.
    let sys = actix::System::new("rugi");

    println!("Trying to start on localhost:{}.", PORT.to_string());
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
                      .method(Method::GET)
                      // have to use .h() for handlers which return `Either`.
                      .h(try_move_with_position)))
        .threads(4) // <- Start 4 workers
        .bind(format!("localhost:{}", PORT)).expect(&format!("Could not bind to port {}", PORT))
        .shutdown_timeout(60)
        .start();  // <- use start_tls() for TLS

    // run the actix system.
    // This will start our server.
    let _ = sys.run();
}
