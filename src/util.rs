use libgitdit::Issue;
use git2::Commit;

use error::GitDitWuiError as GDWE;
use error::*;

pub fn sort_issues_by_time<'a, V>(v: V) -> Result<Vec<Issue<'a>>>
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

pub fn sort_commits_by_time<'a>(mut v: Vec<Commit<'a>>) -> Vec<Commit<'a>> {
    v.sort_by_key(|commit| commit.time());
    v
}
