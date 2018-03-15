use horrorshow::RenderBox;

pub fn render_body_pre() -> Box<RenderBox> {
    box_html! {
        nav(class = "navbar", role = "navigation", aria-label="main navigation") {
            div(class = "navbar-menu is-active") {
                div(class = "navbar-start") {
                    a(class = "navbar-item", href = "/"): "Home";
                    a(class = "navbar-item", href = "/issues"): "Issues";
                }
            }
        }
    }
}

pub fn render_body_post() -> Box<RenderBox> {
    box_html! {
        div(id = ".footer", class = "columns") {
            div(id = "column is-2") {
                : "Made with Love, the gotham web framework, horrorshow-rs and libgitdit"
            }

            div(id = "column is-8") {
            }

            div(id = "column is-2") {
                : "Licensed under AGPL-3.0"
            }
        }
    }
}
