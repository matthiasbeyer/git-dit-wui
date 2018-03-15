use std::iter::FromIterator;

use git2::Commit;
use libgitdit::Issue;

use error::GitDitWuiError as GDWE;
use error::*;

pub fn sort_commits_by_time<'a, V>(mut v: V) -> Result<Vec<Issue<'a>>>
    where V: IntoIterator<Item = Issue<'a>>
{
    let mut res = vec![];

    for element in v.into_iter()
        .map(|i| i.initial_message().map_err(GDWE::from).map(|c| (c.time(), i)))
        .collect::<Vec<Result<_>>>()
    {
        let (time, commit) = element?;
        res.push((time, commit));
    }

    res.sort_by_key(|&(time, _)| time);
    Ok(res.into_iter().map(|(_, c)| c).collect())
}
