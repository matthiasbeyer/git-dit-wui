use error::*;
use renderer::trailer::render_trailer;
use horrorshow::{Raw, RenderBox};
use comrak::{markdown_to_html, ComrakOptions};

use libgitdit::message::block::Block;
use libgitdit::message::Message;

pub fn render_message(c: &::git2::Commit) -> Result<Box<RenderBox>> {
    let id           = format!("{}", c.id());
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
                p {
                    a(href = format!("/message?id={}", id)): id;
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
                    @ for t in trailers {
                        li(id = "trailer-list-item") {
                            : t
                        }
                    }
                }
            }
        }
    })
}

pub fn render_message_text(c: &::libgitdit::message::Message) -> Result<Box<RenderBox>> {
    let mut text     = vec![];

    for block in c.body_blocks() {
        match block {
            Block::Trailer(_) => { /* ignore */ }
            Block::Text(vec)  => for element in vec {
                let buf = markdown_to_html(&element, &ComrakOptions::default());
                text.push(buf);
            },
        }
    };

    Ok(box_html! {
        @for n in text {
            p(id = "message-text-block") {
                : Raw(n)
            }
        }
    })
}

pub fn render_message_trailer_list(c: &::libgitdit::message::Message) -> Result<Box<RenderBox>> {
    let mut trailers = vec![];

    for block in c.body_blocks() {
        match block {
            Block::Text(_)          => { /* ignore */ }
            Block::Trailer(mut vec) => for trailer in vec {
                trailers.push(render_trailer(&trailer)?);
            },
        }
    };

    Ok(box_html! {
        ul(id = "tailer-list") {
            @ for t in trailers {
                li(id = "trailer-list-item") {
                    : t
                }
            }
        }
    })
}

