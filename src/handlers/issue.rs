use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;

use error::GitDitWuiError as GDWE;
use middleware::cache::CacheMiddlewareData;
use middleware::repository::RepositoryMiddlewareData;
use params::extractors::issue::IssueIdExtractor;
use params::extractors::issue::IssueListFilterExtractor;
use params::extractors::issue::IssueFilter;

pub fn index(mut state: State) -> (State, Response) {

    let filter = IssueListFilterExtractor::try_take_from(&mut state)
        .map(|e| e.filter.unwrap_or(IssueFilter::Open))
        .unwrap_or_else(|| IssueFilter::Open);

    debug!("Filter = {:?}", filter);

    let (output, statuscode) = {
        let cache = CacheMiddlewareData::borrow_from(&state);
        let mut issues = cache.issues();
        issues.sort_by_key(|i| *i.date());
        let issues = issues
            .into_iter()
            .filter(|issue| {
                let value = match filter {
                    IssueFilter::All    => true,
                    IssueFilter::Open   => issue.is_open(),
                    IssueFilter::Closed => !issue.is_open(),
                };
                debug!("Issue '{}' => filter: {}", issue.id(), value);
                value
            })
            .rev()
            .collect::<Vec<_>>();

        ::renderer::issue_list::render_issues_list(issues.iter())
            .map(|i| i.into_bytes())
            .map(|x| (x, StatusCode::Ok))
            .unwrap_or_else(|e| {
                (format!("Error: {:?}", e).into_bytes(), StatusCode::InternalServerError)
            })
    };

    let res = create_response(
        &state,
        statuscode,
        Some((output, mime::TEXT_HTML)),
    );

    (state, res)
}

pub fn get_issue_handler(mut state: State) -> (State, Response) {
    let repo = RepositoryMiddlewareData::borrow_from(&state).repo();
    let repo_lock = repo.lock().unwrap();

    let id_param = IssueIdExtractor::take_from(&mut state).id;
    let (output, status) = {
        let cache = CacheMiddlewareData::borrow_from(&state);
        ::git2::Oid::from_str(&id_param)
            .map_err(GDWE::from)
            .map(|oid| (cache.issue(oid), oid))
            .and_then(|(i, oid)| {
                if let Some(i) = i {
                    ::renderer::issue::render_issue(&i, &repo_lock)
                } else {
                    ::renderer::issue::render_issue_not_found(format!("{}", oid))
                }
            })
            .map(|i| i.into_bytes())
            .map(|x| (x, StatusCode::Ok))
            .unwrap_or_else(|e| {
                (format!("Error: {:?}", e).into_bytes(), StatusCode::InternalServerError)
            })
    };

    let res = create_response(
        &state,
        status,
        Some((output, mime::TEXT_HTML)),
    );

    (state, res)
}

