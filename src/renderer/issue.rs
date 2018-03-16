use error::GitDitWuiError as GDWE;
use error::*;
use renderer::message::render_message;

use horrorshow::Template;

pub fn render_issue(i: &::libgitdit::issue::Issue) -> Result<String> {
    let id    = format!("{}", i.id());
    let count = i.messages()?.count();

    let mut messages = vec![];
    for msg in i.messages()? {
        messages.push(render_message(&msg?)?);
    }

    let messages = messages.into_iter().rev().collect::<Vec<_>>();

    (html! {
        html {
            : ::renderer::render_header();
            : ::renderer::render_body_pre();

            div(class = "container") {
                div(class = "content") {
                    header {
                        h1: format!("Issue {}", id);
                    }

                    div(id = "issue") {
                        div(id = "table") {
                            table(class = "table is-striped") {
                                thead {
                                    tr {
                                        th: "Id";
                                        th: "Messages";
                                    }
                                }
                                tbody {
                                    tr {
                                        td {
                                            a(href = format!("/issue?id={}", id)): id;
                                        }
                                        td: count;
                                    }
                                }
                            }
                        }

                        article {
                            @ for msg in messages {
                                : msg
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
