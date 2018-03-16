use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;

use libgitdit::RepositoryExt;

use error::GitDitWuiError as GDWE;
use middleware::repository::RepositoryMiddlewareData;
use params::extractors::issue::IssueIdExtractor;

pub fn index(mut state: State) -> (State, Response) {
    let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
    let repo_lock = repo.lock().unwrap();

    let (output, statuscode) = repo_lock
        .issues()
        .map_err(GDWE::from)
        .and_then(::util::sort_issues_by_time)
        .and_then(|issues| {
            ::renderer::issue_list::render_issues_list(issues.iter().rev())
        })
        .map(|i| i.into_bytes())
        .map(|x| (x, StatusCode::Ok))
        .unwrap_or_else(|e| {
            (format!("Error: {:?}", e).into_bytes(), StatusCode::InternalServerError)
        });

    let res = create_response(
        &state,
        statuscode,
        Some((output, mime::TEXT_HTML)),
    );

    (state, res)
}

pub fn get_issue_handler(mut state: State) -> (State, Response) {
    let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
    let repo_lock = repo.lock().unwrap();

    let id_param = IssueIdExtractor::take_from(&mut state).id;
    let (output, status) = ::git2::Oid::from_str(&id_param)
        .map_err(GDWE::from)
        .and_then(|oid| repo_lock.find_issue(oid).map_err(GDWE::from))
        .and_then(|i| ::renderer::issue::render_issue(&i))
        .map(|i| i.into_bytes())
        .map(|x| (x, StatusCode::Ok))
        .unwrap_or_else(|e| {
            (format!("Error: {:?}", e).into_bytes(), StatusCode::InternalServerError)
        });

    let res = create_response(
        &state,
        status,
        Some((output, mime::TEXT_HTML)),
    );

    (state, res)
}

