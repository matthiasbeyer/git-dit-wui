use error::GitDitWuiError as GDWE;
use error::*;
use renderer::message::render_message;

use horrorshow::Template;

pub fn render_issue(i: &::libgitdit::issue::Issue) -> Result<String> {
    let id    = format!("{}", i.id());
    let count = i.messages()?.count();

    let messages = i.messages()?
        .map(|r| r.map_err(GDWE::from))
        .collect::<Result<_>>()?;

    let messages = ::util::sort_commits_by_time(messages)
        .into_iter()
        .map(|m| render_message(&m))
        .collect::<Result<Vec<_>>>()?;

    (html! {
        html {
            : ::renderer::render_header(vec![], vec![]);
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
