struct MyHandler(usize);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&mut self, req: HttpRequest<S>) -> Self::Result {
        self.0 += 1;
        httpcodes::HttpOk.into()
    }
}

// self.0 value will be different depends on number of threads and number of requests processed per thread. Proper implementation would use Arc and AtomicUsize as follows:

use actix_web::*;
use actix_web::dev::Handler;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct MyHandler(Arc<AtomicUsize>);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&mut self, req: HttpRequest<S>) -> Self::Result {
        self.0.fetch_add(1, Ordering::Relaxed);
        httpcodes::HttpOk.into()
    }
}

fn main() {
    let sys = actix::System::new("example");

    let inc = Arc::new(AtomicUsize::new(0));

    HttpServer::new(
        move || {
            let cloned = inc.clone();
            Application::new()
                .resource("/", move |r| r.h(MyHandler(cloned)))
        })
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
