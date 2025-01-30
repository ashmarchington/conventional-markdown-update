use std::{collections::HashMap, io::Write};

use pulldown_cmark::{Event, LinkType, Parser, Tag, TagEnd};

use crate::{commit::Commit, config::Config};

pub fn parse_markdown(
    base_url: &str,
    file_path: &str,
    after_heading: &str,
    after_level: i32,
    config: Config,
    commits: Vec<Commit>,
) -> Result<(), std::io::Error> {
    let commit_map = build_commit_map(config, commits);
    let markdown = std::fs::read_to_string(file_path)?;

    let mut events: Vec<Event> = Parser::new(markdown.as_str()).collect();

    for (k, v) in &commit_map {
        let mut after_found = false;
        let mut i = 0;
        while i < events.len() {
            if let Event::Start(Tag::Heading { level, .. }) = &events[i] {
                if let Some(Event::Text(text)) = events.get(i + 1) {
                    if after_found && **text == *k {
                        let mut slice = events.split_off(i + 3);
                        let mut append = Vec::with_capacity(v.len() * 10);
                        for c in v {
                            append.extend(vec![
                                Event::Start(Tag::Paragraph),
                                Event::Start(Tag::Link {
                                    link_type: LinkType::Inline,
                                    dest_url: format!("{}/commit/{}", base_url, c.sha).into(),
                                    title: "".into(),
                                    id: "".into(),
                                }),
                                Event::Text(c.sha.clone().into()),
                                Event::End(TagEnd::Link),
                                Event::Text(" ".into()),
                                Event::Code(c.message.clone().into()),
                                Event::Text(" ".into()),
                                Event::Start(Tag::Strong),
                                Event::Text(c.author.clone().into()),
                                Event::End(TagEnd::Strong),
                                Event::Text(" ".into()),
                                Event::Text(c.date.clone().into()),
                                Event::End(TagEnd::Paragraph),
                            ]);
                        }
                        events.append(&mut append);
                        events.append(&mut slice);
                        break;
                    } else if *level as i32 == after_level
                        && !after_found
                        && **text == *after_heading
                    {
                        after_found = true;
                    }
                }
            }
            i += 1;
        }
    }

    let mut buf = String::with_capacity(markdown.len() + 128);
    pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut buf)
        .expect("failed to parse events back to markdown");

    let mut file = std::fs::File::create(file_path).expect("failed to open file for writing");
    file.write_all(buf.as_bytes())
        .expect("failed to write to file");

    Ok(())
}

fn build_commit_map(config: Config, commits: Vec<Commit>) -> HashMap<String, Vec<Commit>> {
    let mut map = HashMap::new();
    for rule in &config.ruleset {
        for commit in &commits {
            if let Some((prefix, _)) = commit.message.split_once(":") {
                if prefix == rule.commit_type {
                    map.entry(rule.header_type.clone())
                        .or_insert_with(Vec::new)
                        .push(commit.clone());
                }
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use crate::config::Rule;

    use super::*;

    #[test]
    fn test_map_build() {
        let config = Config {
            ruleset: vec![Rule {
                commit_type: "test".into(),
                header_type: "tester".into(),
            }],
        };
        let commits = vec![Commit {
            sha: "test-sha".into(),
            date: "2025-01-01".into(),
            message: "test: message".into(),
            author: "me".into(),
        }];

        let mut expected = HashMap::new();
        expected.insert("tester".to_string(), commits.clone());

        assert_eq!(build_commit_map(config, commits), expected);
    }
}
