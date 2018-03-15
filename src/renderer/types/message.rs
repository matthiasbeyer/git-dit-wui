use error::*;
use renderer::types::trailer::render_trailer;
use horrorshow::RenderBox;

use libgitdit::message::block::Block;

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

