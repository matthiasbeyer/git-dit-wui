use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashSet;

use futures::future::Future;
use git2::Repository;
use libgitdit::RepositoryExt;
use gotham::middleware::Middleware;
use gotham::middleware::NewMiddleware;
use gotham::handler::HandlerFuture;
use gotham::state::State;
use chrono::NaiveDateTime;

use error::Result;
use error::GitDitWuiError as GDWE;
use error::GitDitWuiErrorKind as GDWEK;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Issue {
    id: String,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    date: NaiveDateTime,
    is_open: bool,
    number_of_messages: usize,
}

impl Issue {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn author_name(&self) -> &String {
        &self.author_name
    }

    pub fn author_email(&self) -> &String {
        &self.author_email
    }

    pub fn committer_name(&self) -> &String {
        &self.committer_name
    }

    pub fn committer_email(&self) -> &String {
        &self.committer_email
    }

    pub fn date(&self) -> &NaiveDateTime {
        &self.date
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn number_of_messages(&self) -> usize {
        self.number_of_messages
    }
}

struct Cache {
    is_initialized: bool,
    issues: Vec<Issue>,
    number_of_messages: usize,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            is_initialized: false,
            issues: vec![],
            number_of_messages: 0,
        }
    }

    fn set_initialized(&mut self) {
        self.is_initialized = true;
    }

    fn set_issues(&mut self, issues: Vec<Issue>) {
        self.issues = issues;
    }

    fn set_number_of_messages(&mut self, num: usize) {
        self.number_of_messages = num;
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

    pub fn update(&self) -> Result<()> {
        let repo     = self.repo.clone();
        let cache    = self.cache.clone();
        let repolock = repo.lock().unwrap();

        let mut number_of_messages_overall = 0;
        let mut issues = vec![];

        for issue in repolock.issues()? {
            let imsg            = issue.initial_message()?;
            let id              = format!("{}", issue.id());
            let auth            = imsg.author();
            let author_name     = String::from(auth.name().unwrap_or("Unknown author"));
            let author_email    = String::from(auth.email().unwrap_or("Unknown author"));
            let commi           = imsg.committer();
            let committer_name  = String::from(commi.name().unwrap_or("Unknown committer"));
            let committer_email = String::from(commi.email().unwrap_or("Unknown committer"));
            let created         = time_for_commit(&imsg)?;
            let is_open         = ::util::issue_is_open(&issue)?;
            let mut n_msgs      = 0;

            for mess in issue.messages()? {
                let _mess = mess?;
                n_msgs                     += 1;
                number_of_messages_overall += 1;
            }

            issues.push(Issue {
                id,
                author_name,
                author_email,
                committer_name,
                committer_email,
                date: created,
                is_open,
                number_of_messages: n_msgs,
            });
        }

        {
            let mut cachelock = cache.lock().unwrap();
            cachelock.set_issues(issues);
            cachelock.set_number_of_messages(number_of_messages_overall);
            cachelock.set_initialized();
        }

        Ok(())
    }

    pub fn number_of_opened_issues(&self) -> usize {
        let cachelock = self.cache.lock().unwrap();
        let mut num = 0;
        for i in cachelock.issues.iter() {
            if i.is_open() {
                num += 1;
            }
        }
        num
    }

    pub fn number_of_closed_issues(&self) -> usize {
        let cachelock = self.cache.lock().unwrap();
        let mut num = 0;
        for i in cachelock.issues.iter() {
            if !i.is_open() {
                num += 1;
            }
        }
        num
    }

    pub fn number_of_messages(&self) -> usize {
        let cachelock = self.cache.lock().unwrap();
        cachelock.number_of_messages
    }

    pub fn names_of_issue_authors(&self) -> Vec<String> {
        let cachelock      = self.cache.lock().unwrap();
        let mut mails = HashSet::<&String>::new();

        for i in cachelock.issues.iter() {
            mails.insert(i.author_name());
        }

        mails.into_iter().map(Clone::clone).collect()
    }

    pub fn emails_of_issue_authors(&self) -> Vec<String> {
        let cachelock      = self.cache.lock().unwrap();
        let mut mails = HashSet::<&String>::new();

        for i in cachelock.issues.iter() {
            mails.insert(i.author_email());
        }

        mails.into_iter().map(Clone::clone).collect()
    }

    pub fn issue_authors(&self) -> Vec<(String, String)> {
        let cachelock      = self.cache.lock().unwrap();
        let mut committers = HashSet::<(&String, &String)>::new();

        for i in cachelock.issues.iter() {
            committers.insert((i.author_name(), i.author_email()));
        }

        committers.into_iter().map(|(a, b)| (a.clone(), b.clone())).collect()
    }

    pub fn names_of_issue_committers(&self) -> Vec<String> {
        let cachelock      = self.cache.lock().unwrap();
        let mut mails = HashSet::<&String>::new();

        for i in cachelock.issues.iter() {
            mails.insert(i.committer_name());
        }

        mails.into_iter().map(Clone::clone).collect()
    }

    pub fn emails_of_issue_committers(&self) -> Vec<String> {
        let cachelock      = self.cache.lock().unwrap();
        let mut mails = HashSet::<&String>::new();

        for i in cachelock.issues.iter() {
            mails.insert(i.committer_email());
        }

        mails.into_iter().map(Clone::clone).collect()
    }

    pub fn issue_committers(&self) -> Vec<(String, String)> {
        let cachelock      = self.cache.lock().unwrap();
        let mut committers = HashSet::<(&String, &String)>::new();

        for i in cachelock.issues.iter() {
            committers.insert((i.committer_name(), i.committer_email()));
        }

        committers.into_iter().map(|(a, b)| (a.clone(), b.clone())).collect()
    }

    pub fn issues(&self) -> Vec<Issue> {
        let cachelock = self.cache.lock().unwrap();
        let issues    = cachelock.issues.clone();
        issues
    }

    pub fn is_initialized(&self) -> bool {
        let cachelock = self.cache.lock().unwrap();
        cachelock.is_initialized
    }

}

fn time_for_commit<'a>(c: &::git2::Commit<'a>) -> Result<NaiveDateTime> {
    NaiveDateTime::from_timestamp_opt(c.time().seconds(), 0)
        .ok_or_else(|| GDWEK::NoInitialTimeForIssue(format!("{}", c.id())).into())
}

