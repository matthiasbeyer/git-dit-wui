use std::collections::HashMap;
use std::collections::HashSet;

use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;
use chrono::NaiveDateTime;
use libgitdit::RepositoryExt;

use middleware::repository::RepositoryMiddlewareData;
use middleware::cache::CacheMiddlewareData;
use error::GitDitWuiErrorKind as GDWEK;
use error::*;

pub mod issue;
pub mod message;

pub fn index(mut state: State) -> (State, Response) {
    let path = {
        let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
        let repo_lock = repo.lock().unwrap();
        format!("{}", repo_lock.path().to_path_buf().display())
    };

    let (output, status) = {
        let cache = CacheMiddlewareData::borrow_mut_from(&mut state);
        repo_stats(path, &cache)
            .and_then(::renderer::index::render_index)
            .map(|s| (s, StatusCode::Ok))
            .unwrap_or_else(|e| {
                (format!("Failed to process main page: {:?}", e), StatusCode::InternalServerError)
            })
    };

    let res = create_response(
        &state,
        status,
        Some((output.into_bytes(), mime::TEXT_HTML)),
    );

    (state, res)
}

pub struct Stats {
    pub repo_path              : String,
    pub total_number_issues    : usize,
    pub total_number_messages  : usize,
    pub date_issue_count_map   : HashMap<NaiveDateTime, usize>,
    pub authors                : HashSet<String>,
}

fn repo_stats(path: String, cache: &CacheMiddlewareData) -> Result<Stats> {
    let mut total_number_issues    = 0;
    let mut total_number_messages  = 0;
    let mut date_issue_count_map   = HashMap::new();
    let mut authors                = HashSet::new();

    for issue in cache.issues() {
        total_number_issues   += 1;
        total_number_messages += issue.number_of_messages();

        {
            let created = issue.date().clone();
            let count   = date_issue_count_map.entry(created).or_insert(1);
            *count += 1;
        }

        authors.insert(issue.author_name().clone());
    }

    Ok(Stats {
        repo_path              : path,
        total_number_issues    : total_number_issues,
        total_number_messages  : total_number_messages,
        date_issue_count_map   : date_issue_count_map,
        authors                : authors,
    })
}

