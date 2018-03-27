use error::GitDitWuiError as GDWE;
use error::*;

use horrorshow::RenderBox;
use horrorshow::Template;
use chrono::NaiveDateTime;

pub fn render_issues_list<'a, I>(issues: I) -> Result<String>
    where I: Iterator<Item = &'a ::middleware::cache::Issue>
{
    let mut rendered_issues = vec![];
    for issue in issues {
        rendered_issues.push(render_issue(&issue)?);
    }

    (html! {
        html {
            : ::renderer::render_header(vec![], vec![]);

            div(class = "container") {
                div(class = "content") {
                    : ::renderer::render_body_pre();

                    header {
                        h1: "Issues"
                    }

                    div(class = "is-pulled-left") {
                        a(class = "button is-success", href = "/issues?filter=open"): "Open";
                        a(class = "button is-danger", href = "/issues?filter=closed"): "Closed";
                        a(class = "button is-info", href = "/issues?filter=all"): "All";
                    }

                    div(id = "issue") {
                        div(id = "table") {
                            table(class = "table is-striped") {
                                thead {
                                    tr {
                                        th: "Id";
                                        th: "Header";
                                        th: "Metadata";
                                        th: "Created";
                                        th: "Messages";
                                        th: "Status";
                                    }
                                }
                                @ for issue in rendered_issues {
                                    tr {
                                        : issue
                                    }
                                }
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

fn render_issue(i: &::middleware::cache::Issue) -> Result<Box<RenderBox>> {
    let id              = format!("{}", i.id());
    let short_id        = id.chars().take(10).collect::<String>();
    let count           = i.number_of_messages();
    let header          = i.title().clone();
    let author_name     = i.author_name().clone();
    let author_email    = i.author_email().clone();
    let committer_name  = i.committer_name().clone();
    let committer_email = i.committer_email().clone();
    let created         = i.date()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    let status = ::renderer::issue::render_issue_status(i)?;

    Ok(box_html! {
        td {
            a(href = format!("/issue?id={}", id)): short_id;
        }
        td: header;
        td {
            div(class = "dropdown is-hoverable") {
                div(class = "dropdown-trigger") {
                    button(class = "button", aria-haspopup = "true", aria-controls = "dropdown-menu4") {
                        span(class = "icon is-small") {
                            i(class = "fas fa-angle-down", aria-hidden = "true") {
                            }
                        }
                    }
                }
                div(class = "dropdown-menu", id = "dropdown-menu4", role = "menu") {
                    div(class = "dropdown-content") {
                        div(class = "dropdown-item") {
                            p {
                                : "Author: ";
                                : author_name;
                                : " : ";
                                : author_email;
                            }
                        }
                        div(class = "dropdown-item") {
                            p {
                                : "Committer: ";
                                : committer_name;
                                : " : ";
                                : committer_email;
                            }
                        }
                    }
                }
            }
        }
        td: created;
        td: count;
        td: status;
    })
}

