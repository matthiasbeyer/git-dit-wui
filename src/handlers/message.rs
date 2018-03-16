use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;

use error::GitDitWuiError as GDWE;
use middleware::repository::RepositoryMiddlewareData;
use params::extractors::message::MessageIdExtractor;

pub fn get_message_handler(mut state: State) -> (State, Response) {
    let repo = RepositoryMiddlewareData::borrow_mut_from(&mut state).repo();
    let repo_lock = repo.lock().unwrap();

    let id_param = MessageIdExtractor::take_from(&mut state).id;
    let (output, status) = ::git2::Oid::from_str(&id_param)
        .map_err(GDWE::from)
        .and_then(|oid| repo_lock.find_commit(oid).map_err(GDWE::from))
        .and_then(|i| ::renderer::message::render_message_page(&i))
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

