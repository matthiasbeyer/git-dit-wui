use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::Mutex;
use futures::future::Future;

use git2::Repository;
use gotham::middleware::Middleware;
use gotham::middleware::NewMiddleware;
use gotham::handler::HandlerFuture;
use gotham::state::State;

pub struct Issue {
    id: String,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    date: String,
    is_open: bool,
}

struct Cache {
    issues: Vec<Issue>,
    number_of_messages: usize,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            issues: vec![],
            number_of_messages: 0,
        }
    }
}

pub struct CacheMiddleware {
    repo: AssertUnwindSafe<Arc<Mutex<Repository>>>,
    cache: AssertUnwindSafe<Arc<Mutex<Cache>>>,
}

unsafe impl Sync for CacheMiddleware {}

impl CacheMiddleware {
    pub fn new(r: AssertUnwindSafe<Arc<Mutex<Repository>>>) -> CacheMiddleware {
        CacheMiddleware {
            repo: r,
            cache: AssertUnwindSafe(Arc::new(Mutex::new(Cache::new()))),
        }
    }
}

impl Middleware for CacheMiddleware {

    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
        where Chain: FnOnce(State) -> Box<HandlerFuture> + 'static
    {
        let repo = self.repo.clone();
        let cache = self.cache.clone();
        state.put(CacheMiddlewareData {repo, cache});
        chain(state)
    }

}


impl NewMiddleware for CacheMiddleware {
    type Instance = CacheMiddleware;

    fn new_middleware(&self) -> ::std::io::Result<Self::Instance> {
        match (catch_unwind(|| self.repo.clone()), catch_unwind(|| self.cache.clone())) {
            (Ok(r), Ok(c)) => {
                let repo  = AssertUnwindSafe(r);
                let cache = AssertUnwindSafe(c);

                Ok(CacheMiddleware { repo, cache })
            },
            (Err(_), _) | (_, Err(_)) => {
                error!(
                    "PANIC: CacheMiddleware::clone caused a panic, unable to rescue with a HTTP error"
                );
                eprintln!(
                    "PANIC: CacheMiddleware::clone caused a panic, unable to rescue with a HTTP error"
                );
                ::std::process::abort()
            }
        }
    }
}

impl Clone for CacheMiddleware {
    fn clone(&self) -> Self {
        match (catch_unwind(|| self.repo.clone()), catch_unwind(|| self.cache.clone())) {
            (Ok(r), Ok(c)) => {
                let repo  = AssertUnwindSafe(r);
                let cache = AssertUnwindSafe(c);

                CacheMiddleware { repo, cache }
            }
            (Err(_), _) | (_, Err(_)) => {
                error!("PANIC: CacheMiddleware::clone caused a panic");
                eprintln!("PANIC: CacheMiddleware::clone caused a panic");
                ::std::process::abort()
            }
        }
    }
}

#[derive(StateData)]
pub struct CacheMiddlewareData {
    repo: Arc<Mutex<Repository>>,
    cache: Arc<Mutex<Cache>>,
}

impl CacheMiddlewareData {

    pub fn update(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!()
    }

    pub fn number_of_opened_issues(&self) -> usize {
        unimplemented!()
    }

    pub fn number_of_closed_issues(&self) -> usize {
        unimplemented!()
    }

    pub fn number_of_messages(&self) -> usize {
        unimplemented!()
    }

    pub fn number_of_issue_authors(&self) -> usize {
        unimplemented!()
    }

    pub fn names_of_issue_authors(&self) -> Vec<String> {
        unimplemented!()
    }

    pub fn emails_of_issue_authors(&self) -> Vec<String> {
        unimplemented!()
    }

    pub fn issue_authors(&self) -> Vec<(String, String)> {
        unimplemented!()
    }

    pub fn number_of_issue_committers(&self) -> usize {
        unimplemented!()
    }

    pub fn names_of_issue_committers(&self) -> Vec<String> {
        unimplemented!()
    }

    pub fn emails_of_issue_committers(&self) -> Vec<String> {
        unimplemented!()
    }

    pub fn issue_committers(&self) -> Vec<(String, String)> {
        unimplemented!()
    }

    pub fn issues(&self) -> Vec<Issue> {
        unimplemented!()
    }

}



