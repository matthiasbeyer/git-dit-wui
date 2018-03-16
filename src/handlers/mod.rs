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
use error::GitDitWuiErrorKind as GDWEK;
use error::*;

pub mod issue;
pub mod message;

pub fn index(mut state: State) -> (State, Response) {
    let repo = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
    let repo_lock = repo.lock().unwrap();

    let (output, status) = repo_stats(&repo_lock)
        .and_then(::renderer::index::render_index)
        .map(|s| (s, StatusCode::Ok))
        .unwrap_or_else(|e| {
            (format!("Failed to process main page: {:?}", e), StatusCode::InternalServerError)
        });

    let res = create_response(
        &state,
        status,
        Some((output.into_bytes(), mime::TEXT_HTML)),
    );

    (state, res)
}

pub struct Stats {
    pub total_number_issues    : usize,
    pub total_number_messages  : usize,
    pub date_issue_count_map   : HashMap<NaiveDateTime, usize>,
    pub date_message_count_map : HashMap<NaiveDateTime, usize>,
    pub authors                : HashSet<String>,
}

fn repo_stats(r: &::git2::Repository) -> Result<Stats> {
    let mut total_number_issues    = 0;
    let mut total_number_messages  = 0;
    let mut date_issue_count_map   = HashMap::new();
    let mut date_message_count_map = HashMap::new();
    let mut authors                = HashSet::new();

    for issue in r.issues()? {
        total_number_issues += 1;
        {
            let created = time_for_commit(&issue.initial_message()?)?;
            let count   = date_issue_count_map.entry(created).or_insert(1);
            *count += 1;
        }

        for message in issue.messages()? {
            let message = message?;
            total_number_messages += 1;
            {
                let created = time_for_commit(&message)?;
                let count   = date_message_count_map.entry(created).or_insert(1);
                *count += 1;
            }

            let author = message.author();
            if let Some(name) = author.name().map(String::from) {
                let _ = authors.insert(name);
            }
        }
    }

    Ok(Stats {
        total_number_issues    : total_number_issues,
        total_number_messages  : total_number_messages,
        date_issue_count_map   : date_issue_count_map,
        date_message_count_map : date_message_count_map,
        authors                : authors,
    })
}

fn time_for_commit<'a>(c: &::git2::Commit<'a>) -> Result<NaiveDateTime> {
    NaiveDateTime::from_timestamp_opt(c.time().seconds(), 0)
        .ok_or_else(|| GDWEK::NoInitialTimeForIssue(format!("{}", c.id())).into())
}
