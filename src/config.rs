#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Rule {
    pub commit_type: String,
    pub header_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(rename = "commit-rule")]
    pub ruleset: Vec<Rule>,
}

pub fn load_file(path: &str) -> Option<Config> {
    let data = std::fs::read_to_string(path).expect("to be able to read config file");

    toml::from_str(&data).expect("invalid config")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let expected = Config {
            ruleset: vec![
                Rule {
                    commit_type: "feat".into(),
                    header_type: "added".into(),
                },
                Rule {
                    commit_type: "fix".into(),
                    header_type: "fixed".into(),
                },
                Rule {
                    commit_type: "refactor".into(),
                    header_type: "changed".into(),
                },
            ],
        };

        assert_eq!(load_file("./resources/config.toml"), Some(expected));
    }
}
