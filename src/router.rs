use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::Mutex;

use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use git2::Repository;
use hyper::{Get, Head};

use params::extractors::issue::IssueIdExtractor;
use params::extractors::issue::IssueListFilterExtractor;
use params::extractors::message::MessageIdExtractor;
use params::extractors::update::UpdateFlagExtractor;
use middleware::repository::RepositoryMiddleware;
use middleware::cache::CacheMiddleware;

use handlers;

pub fn router(repo: Repository) -> Router {
    let repo                  = Arc::new(Mutex::new(repo));
    let cache_middleware      = CacheMiddleware::new(AssertUnwindSafe(repo.clone()));
    let repository_middleware = RepositoryMiddleware::new(AssertUnwindSafe(repo));

    let pipeline              = new_pipeline()
        .add(repository_middleware)
        .add(cache_middleware)
        .build();

    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route
            .request(vec![Get, Head], "/")
            .with_query_string_extractor::<UpdateFlagExtractor>()
            .to(handlers::index);

        route
            .get_or_head("/issues")
            .with_query_string_extractor::<IssueListFilterExtractor>()
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

