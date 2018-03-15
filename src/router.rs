use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::Mutex;

use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::state::State;
use git2::Repository;
use hyper::{Get, Head};
use handlebars::Handlebars;

use params::extractors::issue::IssueIdExtractor;
use params::extractors::message::MessageIdExtractor;
use middleware::repository::RepositoryMiddleware;
use middleware::handlebars::HandlebarsMiddleware;

use handlers;

pub fn router(repo: Repository, hb: Handlebars) -> Router {
    let repository            = AssertUnwindSafe(Arc::new(Mutex::new(repo)));
    let repository_middleware = RepositoryMiddleware::new(repository);

    let handlebars            = AssertUnwindSafe(Arc::new(Mutex::new(hb)));
    let handlebars_middleware = HandlebarsMiddleware::new(handlebars);
    let pipeline              = new_pipeline()
        .add(repository_middleware)
        .add(handlebars_middleware)
        .build();

    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route
            .request(vec![Get, Head], "/")
            .to(handlers::index);

        route
            .get_or_head("/issues")
            .to(handlers::issue::index);

        route
            .get_or_head("/issue")
            .with_query_string_extractor::<IssueIdExtractor>()
            .to(handlers::issue::get_issue_handler);

        route
            .get_or_head("/message")
            .with_query_string_extractor::<MessageIdExtractor>()
            .to(handlers::message::get_message_handler);
    })
}

