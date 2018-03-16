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

pub fn issue_is_open<'a>(i: &Issue<'a>) -> Result<bool> {
    use libgitdit::trailer::accumulation;
    use libgitdit::trailer::accumulation::Accumulator;
    use libgitdit::trailer::TrailerValue;
    use libgitdit::Message;

    let policy  = accumulation::AccumulationPolicy::Latest;
    let mut acc = accumulation::SingleAccumulator::new("Dit-status".to_owned(), policy);

    let mut trailers = vec![];
    for message in i.messages()? {
        let mut message_trailers = message?.trailers().collect();
        trailers.append(&mut message_trailers);
    }

    acc.process_all(trailers.into_iter());


    if let Some((_, val)) = acc.into_iter().next() {
        match val {
            TrailerValue::String(s) => {
                debug!("Trailer value: '{}'", s);
                return Ok(s == "OPEN" || s == "open" || s == "Open");
            },
            _ => {},
        }
    }

    return Ok(false);
}

pub fn issue_is_closed<'a>(i: &Issue<'a>) -> Result<bool> {
    return issue_is_open(i).map(|x| !x);
}

