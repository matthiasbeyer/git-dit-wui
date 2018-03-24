use error::GitDitWuiError as GDWE;
use error::*;
use renderer::message::render_message;

use horrorshow::RenderBox;
use horrorshow::Template;

pub fn render_issue(i: &::libgitdit::issue::Issue) -> Result<String> {
    let id    = format!("{}", i.id());
    let count = i.messages()?.count();

    let messages = i.messages()?
        .map(|r| r.map_err(GDWE::from))
        .collect::<Result<_>>()?;

    let issue_header = String::from(i.initial_message()?.summary().unwrap_or("<empty>"));

    let messages = ::util::sort_commits_by_time(messages)
        .into_iter()
        .map(|m| render_message(&m))
        .collect::<Result<Vec<_>>>()?;

    let status = render_issue_status(i)?;

    (html! {
        html {
            : ::renderer::render_header(vec![], vec![]);

            div(class = "container") {
                div(class = "content") {
                    : ::renderer::render_body_pre();

                    header {
                        h1: issue_header;
                    }

                    div(id = "issue") {
                        div(id = "table") {
                            table(class = "table is-striped") {
                                thead {
                                    tr {
                                        th: "Id";
                                        th: "Messages";
                                        th: "Status";
                                    }
                                }
                                tbody {
                                    tr {
                                        td {
                                            a(href = format!("/issue?id={}", id)): id;
                                        }
                                        td: count;
                                        td: status;
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
                    : ::renderer::render_body_post();
                }
            }

            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}

pub fn render_issue_status(i: &::libgitdit::issue::Issue) -> Result<Box<RenderBox>> {
    if ::util::issue_is_open(i)? {
        Ok(box_html! { span(class = "tag is-success"): "Open"; })
    } else {
        Ok(box_html! { span(class = "tag is-danger"): "Closed"; })
    }
}

