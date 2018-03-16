use error::GitDitWuiError as GDWE;
use error::*;

use horrorshow::RenderBox;
use horrorshow::Template;
use chrono::NaiveDateTime;

pub fn render_issues_list<'a, I>(issues: I) -> Result<String>
    where I: Iterator<Item = &'a ::libgitdit::issue::Issue<'a>>
{
    let mut rendered_issues = vec![];
    for issue in issues {
        rendered_issues.push(render_issue(issue)?);
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
                                        th: "Author Name";
                                        th: "Author Email";
                                        th: "Committer Name";
                                        th: "Committer Email";
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

fn render_issue(i: &::libgitdit::issue::Issue) -> Result<Box<RenderBox>> {
    let id              = format!("{}", i.id());
    let count           = i.messages()?.count();
    let initial         = i.initial_message()?;

    let author          = initial.author();
    let author_name     = String::from(author.name().unwrap_or("Unknown name"));
    let author_email    = String::from(author.email().unwrap_or("Unknown email"));

    let committer       = initial.committer();
    let committer_name  = String::from(committer.name().unwrap_or("Unknown name"));
    let committer_email = String::from(committer.email().unwrap_or("Unknown email"));

    let created        = match NaiveDateTime::from_timestamp_opt(initial.time().seconds(), 0) {
        Some(ts) => ts.format("%Y-%m-%d %H:%M:%S").to_string(),
        None     => String::from("Time format wrong"),
    };

    let status = ::renderer::issue::render_issue_status(i)?;

    Ok(box_html! {
        td {
            a(href = format!("/issue?id={}", id)): id;
        }
        td: author_name;
        td: author_email;
        td: committer_name;
        td: committer_email;
        td: created;
        td: count;
        td: status;
    })
}

