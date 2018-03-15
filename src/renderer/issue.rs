use error::GitDitWuiError as GDWE;
use error::*;
use renderer::message::render_message_text;
use renderer::message::render_message_trailer_list;

use horrorshow::RenderBox;
use horrorshow::Template;

pub fn render_issue(i: &::libgitdit::issue::Issue) -> Result<String> {
    let id    = format!("{}", i.id());
    let count = i.messages()?.count();

    let mut messages = vec![];
    for msg in i.messages()? {
        let msg = msg?;
        messages.push((render_message_text(&msg)?, render_message_trailer_list(&msg)?));
    }

    (html! {
        html {
            : ::renderer::render_header();
            : ::renderer::render_body_pre();

            header {
                h1: format!("Issue {}", id);
            }

            main {
                div(id = "issue") {
                    div(id = "issue-meta") {
                        p(id = "issue-meta-message-count"): count;
                        p(id = "issue-meta-id") {
                            a(href = format!("/issue=id={}", id)): id
                        }
                    }

                    div(id = "issue-messages") {
                        @ for (text, trailers) in messages {
                            div(id = "message") {
                                div(id = "message-text"): text;
                                div(id = "message-trailers"): trailers;
                            }
                        }
                    }
                }
            }

            : ::renderer::render_body_post();
            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}
