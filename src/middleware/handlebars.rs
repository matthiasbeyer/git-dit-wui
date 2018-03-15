use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::Mutex;

use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::middleware::NewMiddleware;
use gotham::state::State;
use handlebars::Handlebars;

pub struct HandlebarsMiddleware(AssertUnwindSafe<Arc<Mutex<Handlebars>>>);

unsafe impl Sync for HandlebarsMiddleware {}

impl HandlebarsMiddleware {
    pub fn new(r: AssertUnwindSafe<Arc<Mutex<Handlebars>>>) -> HandlebarsMiddleware {
        HandlebarsMiddleware(r)
    }
}

impl Middleware for HandlebarsMiddleware {

    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
        where Chain: FnOnce(State) -> Box<HandlerFuture> + 'static
    {
        let a = self.0.clone();
        state.put(HandlebarsMiddlewareData { hb: a });
        chain(state)
    }

}


impl NewMiddleware for HandlebarsMiddleware {
    type Instance = HandlebarsMiddleware;

    fn new_middleware(&self) -> ::std::io::Result<Self::Instance> {
        match catch_unwind(|| self.0.clone()) {
            Ok(r) => Ok(HandlebarsMiddleware(AssertUnwindSafe(r))),
            Err(_) => {
                error!(
                    "PANIC: HandlebarsMiddleware::clone caused a panic, unable to rescue with a HTTP error"
                );
                eprintln!(
                    "PANIC: HandlebarsMiddleware::clone caused a panic, unable to rescue with a HTTP error"
                );
                ::std::process::abort()
            }
        }
    }
}

impl Clone for HandlebarsMiddleware {
    fn clone(&self) -> Self {
        match catch_unwind(|| self.0.clone()) {
            Ok(r) => HandlebarsMiddleware(AssertUnwindSafe(r)),
            Err(_) => {
                error!("PANIC: HandlebarsMiddleware::clone caused a panic");
                eprintln!("PANIC: HandlebarsMiddleware::clone caused a panic");
                ::std::process::abort()
            }
        }
    }
}

#[derive(StateData)]
pub struct HandlebarsMiddlewareData {
    hb: Arc<Mutex<Handlebars>>,
}

impl HandlebarsMiddlewareData {
    pub fn handlebars(&self) -> Arc<Mutex<Handlebars>> {
        self.hb.clone()
    }
}


