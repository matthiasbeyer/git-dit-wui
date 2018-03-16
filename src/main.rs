#[macro_use] extern crate error_chain;
#[macro_use] extern crate gotham_derive;
#[macro_use] extern crate horrorshow;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate comrak;
extern crate env_logger;
extern crate git2;
extern crate gotham;
extern crate hyper;
extern crate libgitdit;
extern crate mime;
extern crate serde;

use std::path::PathBuf;
use git2::Repository;

mod error;
mod handlers;
mod middleware;
mod params;
mod renderer;
mod router;
mod util;

fn main() {
    ::env_logger::init();
    info!("Logger is up");
    let repository = Repository::open(PathBuf::from(".")).unwrap();

    let addr = "127.0.0.1:7878";
    gotham::start(addr, router::router(repository))
}

