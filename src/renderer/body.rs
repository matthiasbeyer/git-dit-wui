use horrorshow::RenderBox;

pub fn render_body_pre() -> Box<RenderBox> {
    box_html! {
        div(id = "nav") {
            a(href = "/"): "Home";
            a(href = "/issues"): "Issues";
        }
    }
}

pub fn render_body_post() -> Box<RenderBox> {
    box_html! {
        div(id = "footer") {
            div(id = "attribution") {
                : "Made with Love, the gotham web framework, horrorshow-rs and libgitdit"
            }

            div(id = "version") {
                : "Licensed under AGPL-3.0"
            }
        }
    }
}
