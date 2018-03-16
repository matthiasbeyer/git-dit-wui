use horrorshow::RenderBox;

pub const CHART_JS_SRC : &'static str = "https://cdnjs.cloudflare.com/ajax/libs/Chart.js/2.7.2/Chart.min.js";

pub fn render_header(additional_links: Vec<String>, additional_scripts: Vec<String>) -> Box<RenderBox> {
    box_html! {
        head {
            meta(charset = "UTF-8");
            meta(name = "viewport", content="width=device-width, initial-scale=1");
            link(rel = "stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.6.2/css/bulma.min.css");

            @ for element in additional_links {
                link(rel = "stylesheet", href = element);
            }

            script(defer, src = "https://use.fontawesome.com/releases/v5.0.6/js/all.js") {
            }

            @ for element in additional_scripts {
                script(defer, src = element) {
                }
            }
        }
    }
}

