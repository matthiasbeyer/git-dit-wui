use error::*;
use horrorshow::RenderBox;

pub fn render_trailer(t: &::libgitdit::trailer::Trailer) -> Result<Box<RenderBox>> {
    let key = format!("{}", t.key);
    let val = format!("{}", t.value);
    Ok(box_html! {
        td: key;
        td: val;
    })
}
