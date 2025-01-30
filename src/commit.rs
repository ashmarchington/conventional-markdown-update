use core::str::FromStr;

use crate::ActionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub sha: String,
    pub date: String,
    pub message: String,
    pub author: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCommitError;

impl core::fmt::Display for ParseCommitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "parsing failure")
    }
}

impl FromStr for Commit {
    type Err = ParseCommitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [sha, date, message, author] =
            &*s.split(',').map(String::from).collect::<Vec<String>>()
        {
            return Ok(Self {
                sha: sha.to_owned(),
                date: date.to_owned(),
                message: message.to_owned(),
                author: author.to_owned(),
            });
        }

        Err(ParseCommitError)
    }
}

pub fn load_file(path: String) -> Result<Vec<Commit>, ActionError<'static>> {
    let commit_data = match std::fs::read_to_string(path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return Err(ActionError::Commit("failed to read commit file"));
        }
    };

    let commits = commit_data
        .split('\n')
        .filter_map(|l| Commit::from_str(l).ok())
        .collect::<Vec<Commit>>();

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let expected = vec![
            Commit {
                sha: "824687e4715aec421c7f4651ba60f82cb8dfae8b".into(),
                date: "2025-01-28".into(),
                message: "fix: everything".into(),
                author: "ashmarch".into(),
            },
            Commit {
                sha: "824687e4715aec421c7f4651ba60f82cb8dfae8b".into(),
                date: "2025-01-27".into(),
                message: "fix: some stuff".into(),
                author: "ashmarch".into(),
            },
            Commit {
                sha: "30ce9c2edc1696b956898d4f4b273e7eb8e15516".into(),
                date: "2025-01-28".into(),
                message: "feat: all of it".into(),
                author: "ashmarch".into(),
            },
            Commit {
                sha: "30ce9c2edc1696b956898d4f4b273e7eb8e15516".into(),
                date: "2025-01-26".into(),
                message: "feat: none of it".into(),
                author: "ashmarch".into(),
            },
        ];

        let actual = load_file("./resources/log.txt".into());
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap_or_default(), expected);
    }
}
