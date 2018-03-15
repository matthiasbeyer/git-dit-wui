use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;

use libgitdit::RepositoryExt;
use libgitdit::Message;

use error::*;
use error::GitDitWuiError as GDWE;
use middleware::repository::RepositoryMiddlewareData;

pub fn index(mut state: State) -> (State, Response) {
    let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
    let repo_lock = repo.lock().unwrap();

    let (output, statuscode) = repo_lock
        .issues()
        .map_err(GDWE::from)
        .and_then(|issues| {
            ::renderer::types::issue::render_issues_list(issues.iter())
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
    //let repo = RepositoryMiddlewareData::borrow_mut_from(&mut state);

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Showing an issue!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

