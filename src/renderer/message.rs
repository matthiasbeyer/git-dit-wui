use error::*;
use error::GitDitWuiError as GDWE;
use renderer::trailer::render_trailer;

use horrorshow::{Raw, RenderBox};
use horrorshow::Template;
use comrak::{markdown_to_html, ComrakOptions};
use chrono::NaiveDateTime;
use libgitdit::message::block::Block;
use libgitdit::message::Message;

pub fn render_message_page(c: &::git2::Commit) -> Result<String> {
    let rendered_message = render_message(c)?;
    (html! {
        html {
            : ::renderer::render_header(vec![], vec![]);
            : ::renderer::render_body_pre();

            div(class = "container") {
                div(class = "content") {
                    : rendered_message;
                }
            }

            : ::renderer::render_body_post();
            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}

pub fn render_message(c: &::git2::Commit) -> Result<Box<RenderBox>> {
    let id            = format!("{}", c.id());
    let author        = c.author();
    let author_name   = String::from(author.name().unwrap_or("Unknown name"));
    let author_email  = String::from(author.email().unwrap_or("Unknown email"));
    let opt_committer = String::from(c.committer().name().unwrap_or(""));
    let created       = match NaiveDateTime::from_timestamp_opt(c.time().seconds(), 0) {
        Some(ts) => ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        None     => String::from("Time format wrong"),
    };

    let mut text     = vec![];
    let mut trailers = vec![];

    for block in c.body_blocks() {
        match block {
            Block::Text(vec) => for element in vec {
                let buf = markdown_to_html(&element, &ComrakOptions::default());
                text.push(buf);
            }
            Block::Trailer(mut vec) => for trailer in vec {
                trailers.push(render_trailer(&trailer)?);
            },
        }
    };

    Ok(box_html! {
        div(class = "message") {
            div(class = "message-header") {
                p(class = "is-pulled-left") {
                    a(href = format!("/message?id={}", id)): id;
                }
                p(class = "is-pulled-left") {
                    : author_name;
                }
                p(class = "is-pulled-left") {
                    : author_email;
                }
                p(class = "is-pulled-left") {
                    : opt_committer;
                }
                p(class = "is-pulled-right") {
                    : created;
                }
            }
            div(class = "message-body") {
                div(id = "message-text") {
                    @for n in text {
                        p(id = "message-text-block") {
                            : Raw(n)
                        }
                    }
                }
                ul(id = "message-trailers") {
                    table(class = "table is-striped") {
                        thead {
                            tr {
                                th: "Trailer";
                                th: "Value";
                            }
                        }
                        @ for t in trailers {
                            tr {
                                : t;
                            }
                        }
                    }
                }
            }
        }
    })
}

