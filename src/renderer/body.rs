use horrorshow::RenderBox;

pub fn render_body_pre() -> Box<RenderBox> {
    box_html! {
        nav(class = "navbar", role = "navigation", aria-label="main navigation") {
            div(class = "navbar-menu is-active") {
                div(class = "navbar-start") {
                    a(class = "navbar-item", href = "/"): "Home";
                    a(class = "navbar-item", href = "/issues"): "Issues";
                    a(class = "navbar-item", href = "/?update=true"): "Update cache";
                }
            }
        }
    }
}

pub fn render_body_post() -> Box<RenderBox> {
    box_html! {
        footer(class = "footer") {
            div(class = "container") {
                div(class = "content has-text-centered") {
                    p {
                        strong: "git-dit-wui";
                    }
                    p: "Made with Love, the gotham web framework, horrorshow-rs and libgitdit by Matthias Beyer";
                    p: "Licensed under AGPL-3.0";
                }
            }
        }
    }
}
