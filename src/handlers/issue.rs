use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;
use handlebars::Handlebars;

use libgitdit::RepositoryExt;
use libgitdit::Message;

use middleware::repository::RepositoryMiddlewareData;
use renderer::types::issue::IssueListItem;

pub fn index(mut state: State) -> (State, Response) {
    let output = {
        let repo      = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
        let repo_lock = repo.lock().unwrap();
        let data      = match repo_lock.issues() {
            Ok(issues) => {
                let data = {
                    let i = issues
                        .iter()
                        .map(IssueListItem::from_issue)
                        .map(Result::unwrap)
                        .collect::<Vec<IssueListItem>>();
                    let mut b = ::std::collections::BTreeMap::new();
                    b.insert("issues", i);
                    b
                };

                let mut reg = Handlebars::new();
                reg.render_template(include_str!("../templates/issues.hbs"), &data)
                    .unwrap()
            },

            Err(e) => {
                String::from("Failed to get issues")
            }
        };

        data.into_bytes()
    };

    let res = create_response(
        &state,
        StatusCode::Ok,
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
