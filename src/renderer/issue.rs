use error::GitDitWuiError as GDWE;
use error::*;
use renderer::message::render_message;

use git2::Repository;
use horrorshow::RenderBox;
use horrorshow::Template;

pub fn render_issue(i: &::middleware::cache::Issue, repo: &Repository) -> Result<String> {
    let id           = i.id().clone();
    let count        = i.number_of_messages();
    let issue_header = i.title();
    let status       = render_issue_status(&i)?;

    let messages = i
        .message_ids()
        .iter()
        .map(|oid| repo.find_commit(*oid).map_err(GDWE::from))
        .collect::<Result<_>>()?;


    let messages = ::util::sort_commits_by_time(messages)
        .into_iter()
        .map(|m| render_message(&m))
        .collect::<Result<Vec<_>>>()?;

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

pub fn render_issue_not_found(i: String) -> Result<String> {
    (html! {
        html {
            : ::renderer::render_header(vec![], vec![]);

            div(class = "container") {
                div(class = "content") {
                    : ::renderer::render_body_pre();

                    header {
                        h1 {
                            :"Issue not found: ";
                            :i;
                        }
                    }

                    : ::renderer::render_body_post();
                }
            }

            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}

pub fn render_issue_status(i: &::middleware::cache::Issue) -> Result<Box<RenderBox>> {
    if i.is_open() {
        Ok(box_html! { span(class = "tag is-success"): "Open"; })
    } else {
        Ok(box_html! { span(class = "tag is-danger"): "Closed"; })
    }
}

