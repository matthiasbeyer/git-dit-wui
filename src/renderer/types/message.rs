use error::*;
use renderer::types::trailer::Trailer;
use renderer::types::text_block::TextBlock;

use libgitdit::message::block::Block;

#[derive(Serialize)]
pub struct Message {
    blocks: Vec<TextBlock>,
    trailers: Vec<Trailer>
}

impl Message {
    pub fn from_message(m: &::libgitdit::message::Message) -> Result<Message> {
        let mut text     = vec![];
        let mut trailers = vec![];

        for block in m.body_blocks() {
            match block {
                Block::Text(vec) => {
                    text.push(TextBlock::new(vec.join(" ")));
                },
                Block::Trailer(vec) => for trailer in vec.iter() {
                    let t = Trailer::from_trailer(trailer)?;
                    trailers.push(t);
                },
            }
        };

        Ok(Message {
            blocks: text,
            trailers: trailers,
        })
    }
}
