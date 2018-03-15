use error::*;
use renderer::trailer::render_trailer;
use horrorshow::RenderBox;

use libgitdit::message::block::Block;
use libgitdit::message::Message;

pub fn render_message(c: &::git2::Commit) -> Result<Box<RenderBox>> {
    let id           = format!("{}", c.id());
    let mut text     = vec![];
    let mut trailers = vec![];

    for block in c.body_blocks() {
        match block {
            Block::Text(mut vec) => text.append(&mut vec),
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
                            : n
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
            Block::Trailer(vec)  => { /* ignore */ }
            Block::Text(mut vec) => text.append(&mut vec),
        }
    };

    Ok(box_html! {
        @for n in text {
            p(id = "message-text-block") {
                : n
            }
        }
    })
}

pub fn render_message_trailer_list(c: &::libgitdit::message::Message) -> Result<Box<RenderBox>> {
    let mut trailers = vec![];

    for block in c.body_blocks() {
        match block {
            Block::Text(vec)        => { /* ignore */ }
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

