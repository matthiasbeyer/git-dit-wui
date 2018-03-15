use horrorshow::RenderBox;

pub fn render_header() -> Box<RenderBox> {
    box_html! {
        head {
            meta(charset = "UTF-8");
            meta(name = "viewport", content="width=device-width, initial-scale=1");
            link(rel = "stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.6.2/css/bulma.min.css");
            script(defer, src = "https://use.fontawesome.com/releases/v5.0.6/js/all.js") {
            }
        }
    }
}

