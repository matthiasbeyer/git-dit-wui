use error::GitDitWuiError as GDWE;
use error::*;
use renderer::types::message::Message;
use renderer::types::text_block::TextBlock;
use renderer::types::trailer::Trailer;

#[derive(Serialize)]
pub struct IssueListItem {
    id: String,
    initial_message: Message,
    count_messages: usize,
}

impl IssueListItem {
    pub fn from_issue<'r>(i: &'r ::libgitdit::issue::Issue) -> Result<IssueListItem> {
        let count = i.messages()?.count();
        let ini   = i.initial_message().map_err(GDWE::from).and_then(|c| Message::from_message(&c))?;

        Ok(IssueListItem {
            id: format!("{}", i.id()),
            initial_message: ini,
            count_messages: count,
        })
    }
}
