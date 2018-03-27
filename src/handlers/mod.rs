use std::collections::HashMap;
use std::collections::HashSet;

use gotham::state::State;
use gotham::state::FromState;
use gotham::handler::HandlerFuture;
use gotham::handler::IntoHandlerError;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;
use chrono::NaiveDateTime;
use libgitdit::RepositoryExt;
use futures::Future;

use middleware::repository::RepositoryMiddlewareData;
use middleware::cache::CacheMiddlewareData;
use params::extractors::update::UpdateFlagExtractor;
use error::GitDitWuiErrorKind as GDWEK;
use error::*;

pub mod issue;
pub mod message;

pub fn index(mut state: State) -> Box<HandlerFuture> {
    info!("Index...");
    let path = {
        let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
        let repo_lock = repo.lock().unwrap();
        format!("{}", repo_lock.path().to_path_buf().display())
    };

    let cache_is_initialized = CacheMiddlewareData::borrow_from(&state).is_initialized();

    let do_update_cache = !cache_is_initialized || UpdateFlagExtractor::try_take_from(&mut state)
        .map(|e| e.update.unwrap_or(false))
        .unwrap_or(false);

    info!("Do update cache: {}", do_update_cache);

    let fut = ::futures::future::ok(())
        .and_then(move |_: ()| {
            let mut state = state;
            if do_update_cache {
                info!("Aggregating cache before using");
                match CacheMiddlewareData::borrow_from(&state).update() {
                    Ok(_)  => Ok(state),
                    Err(e) => Err((state, e.into_handler_error())),
                }
            } else {
                info!("Not aggregating cache before using it");
                Ok(state)
            }
        })
        .map(|mut state| {
            let (output, status) = {
                let cache = CacheMiddlewareData::borrow_mut_from(&mut state);
                info!("Collecting repo statistics");
                repo_stats(path, &cache)
                    .and_then(::renderer::index::render_index)
                    .map(|s| (s, StatusCode::Ok))
                    .unwrap_or_else(|e| {
                        (format!("Failed to process main page: {:?}", e), StatusCode::InternalServerError)
                    })
            };

            info!("Creating response");
            let res = create_response(
                &state,
                status,
                Some((output.into_bytes(), mime::TEXT_HTML)),
            );

            (state, res)
        });

    Box::new(fut)
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

