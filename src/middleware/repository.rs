use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::Mutex;

use git2::Repository;
use gotham::middleware::Middleware;
use gotham::middleware::NewMiddleware;
use gotham::handler::HandlerFuture;
use gotham::state::State;

pub struct RepositoryMiddleware(AssertUnwindSafe<Arc<Mutex<Repository>>>);

unsafe impl Sync for RepositoryMiddleware {}

impl RepositoryMiddleware {
    pub fn new(r: AssertUnwindSafe<Arc<Mutex<Repository>>>) -> RepositoryMiddleware {
        RepositoryMiddleware(r)
    }
}

impl Middleware for RepositoryMiddleware {

    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
        where Chain: FnOnce(State) -> Box<HandlerFuture> + 'static
    {
        let a = self.0.clone();
        state.put(RepositoryMiddlewareData { repo: a });
        chain(state)
    }

}


impl NewMiddleware for RepositoryMiddleware {
    type Instance = RepositoryMiddleware;

    fn new_middleware(&self) -> ::std::io::Result<Self::Instance> {
        match catch_unwind(|| self.0.clone()) {
            Ok(r) => Ok(RepositoryMiddleware(AssertUnwindSafe(r))),
            Err(_) => {
                error!(
                    "PANIC: RepositoryMiddleware::clone caused a panic, unable to rescue with a HTTP error"
                );
                eprintln!(
                    "PANIC: RepositoryMiddleware::clone caused a panic, unable to rescue with a HTTP error"
                );
                ::std::process::abort()
            }
        }
    }
}

impl Clone for RepositoryMiddleware {
    fn clone(&self) -> Self {
        match catch_unwind(|| self.0.clone()) {
            Ok(r) => RepositoryMiddleware(AssertUnwindSafe(r)),
            Err(_) => {
                error!("PANIC: RepositoryMiddleware::clone caused a panic");
                eprintln!("PANIC: RepositoryMiddleware::clone caused a panic");
                ::std::process::abort()
            }
        }
    }
}

#[derive(StateData)]
pub struct RepositoryMiddlewareData {
    repo: Arc<Mutex<Repository>>,
}

impl RepositoryMiddlewareData {
    pub fn repo(&self) -> Arc<Mutex<Repository>> {
        self.repo.clone()
    }
}


