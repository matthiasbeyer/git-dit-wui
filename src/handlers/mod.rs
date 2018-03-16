use gotham::state::State;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;

pub fn index(state: State) -> (State, Response) {
    //let repo = RepositoryMiddlewareData::borrow_mut_from(&mut state);

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Index!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

pub mod issue;
pub mod message;
