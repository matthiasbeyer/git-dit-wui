use gotham::state::State;
use gotham::state::FromState;
use gotham::http::response::create_response;
use hyper::{Response, StatusCode};
use mime;

use middleware::repository::RepositoryMiddlewareData;

pub fn get_message_handler(mut state: State) -> (State, Response) {
    //let repo = RepositoryMiddlewareData::borrow_mut_from(&mut state);

    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Showing a message!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

