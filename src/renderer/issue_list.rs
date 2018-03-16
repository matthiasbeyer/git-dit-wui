use error::GitDitWuiError as GDWE;
use error::*;

use horrorshow::RenderBox;
use horrorshow::Template;

pub fn render_issues_list<'a, I>(issues: I) -> Result<String>
    where I: Iterator<Item = &'a ::libgitdit::issue::Issue<'a>>
{
    let mut rendered_issues = vec![];
    for issue in issues {
        rendered_issues.push(render_issue(issue)?);
    }

    (html! {
        html {
            : ::renderer::render_header();
            : ::renderer::render_body_pre();

            header {
                h1: "Issues"
            }

            div(class = "container") {
                div(class = "content") {
                    div(id = "issue") {
                        div(id = "table") {
                            table(class = "table is-striped") {
                                thead {
                                    tr {
                                        th: "Id";
                                        th: "Messages";
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
                }
            }

            : ::renderer::render_body_post();
            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}

fn render_issue(i: &::libgitdit::issue::Issue) -> Result<Box<RenderBox>> {
    let id    = format!("{}", i.id());
    let count = i.messages()?.count();

    Ok(box_html! {
        td {
            a(href = format!("/issue?id={}", id)): id;
        }
        td: count;
    })
}

