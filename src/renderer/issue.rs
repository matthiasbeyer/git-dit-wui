use error::GitDitWuiError as GDWE;
use error::*;
use renderer::message::render_message_text;
use renderer::message::render_message_trailer_list;

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

            @ for issue in rendered_issues {
                : issue
            }

            : ::renderer::render_body_post();
            : ::renderer::render_footer();
        }
    }).into_string().map_err(GDWE::from)
}

fn render_issue(i: &::libgitdit::issue::Issue) -> Result<Box<RenderBox>> {
    let id    = format!("{}", i.id());
    let count = i.messages()?.count();
    let ini   = i
        .initial_message()
        .map_err(GDWE::from)
        .and_then(|c| render_message_text(&c))?;

    Ok(box_html! {
        div(id = "issue") {
            div(id = "issue-meta") {
                p(id = "issue-meta-message-count"): count;
                p(id = "issue-meta-id") {
                    a(href = format!("/issue=id={}", id)): id
                }
            }
            div(id = "issue-message-text") {
                : ini
            }
        }
    })
}
