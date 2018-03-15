use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;
use handlebars::Handlebars;

use libgitdit::RepositoryExt;
use libgitdit::Message;

use error::*;
use error::GitDitWuiError as GDWE;
use middleware::repository::RepositoryMiddlewareData;
use middleware::handlebars::HandlebarsMiddlewareData;
use renderer::types::issue::IssueListItem;

pub fn index(mut state: State) -> (State, Response) {
    let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
    let repo_lock = repo.lock().unwrap();

    let hb      = HandlebarsMiddlewareData::borrow_from(&mut state).handlebars();
    let hb_lock = hb.lock().unwrap();

    let (output, statuscode) = repo_lock
        .issues()
        .map_err(GDWE::from)
        .and_then(|issues| {
            issues.iter()
                .map(IssueListItem::from_issue)
                .collect::<Result<Vec<IssueListItem>>>()
                .and_then(|issues| {
                    let mut data = ::std::collections::BTreeMap::new();
                    data.insert("issues", issues);
                    hb_lock.render("issue_list", &data)
                        .map_err(GDWE::from)
                })
                .map(|s| s.into_bytes())
        })
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

