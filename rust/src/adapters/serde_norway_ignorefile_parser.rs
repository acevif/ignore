use crate::domain::IgnoreConfig;
use crate::ports::{IgnorefileParseError, IgnorefileParser};
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Clone, Default)]
pub struct SerdeNorwayIgnorefileParser;

#[derive(Debug, Clone, Default, Deserialize)]
struct IgnorefileYaml {
    #[serde(rename = "gitignore.io", default)]
    gitignore_io: Vec<String>,

    #[serde(default)]
    github: Vec<String>,

    #[serde(rename = "paths-ignore", default)]
    paths_ignore: Vec<String>,
}

impl From<IgnorefileYaml> for IgnoreConfig {
    fn from(value: IgnorefileYaml) -> Self {
        Self {
            gitignore_io: value.gitignore_io,
            github: value.github,
            paths_ignore: value.paths_ignore,
        }
    }
}

impl IgnorefileParser for SerdeNorwayIgnorefileParser {
    fn parse<R: Read>(&self, reader: R) -> Result<IgnoreConfig, IgnorefileParseError> {
        let parsed: IgnorefileYaml = serde_norway::from_reader(reader)
            .map_err(|e| IgnorefileParseError::new(e.to_string()))?;
        Ok(parsed.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::parse_ignorefile;

    #[test]
    fn parses_ignorefile_yaml() {
        let input = r#"
gitignore.io:
  - Ruby
  - direnv
github:
  - Python
paths-ignore:
  - "foo"
  - "bar"
  - "!baz"
"#;

        let parser = SerdeNorwayIgnorefileParser;
        let parsed = parse_ignorefile(&parser, input.as_bytes()).expect("parse Ignorefile YAML");

        assert_eq!(
            parsed,
            IgnoreConfig {
                gitignore_io: vec!["Ruby".to_string(), "direnv".to_string()],
                github: vec!["Python".to_string()],
                paths_ignore: vec!["foo".to_string(), "bar".to_string(), "!baz".to_string()],
            }
        );
    }

    #[test]
    fn parses_ignorefile_with_empty_paths_ignore() {
        let input = r#"
paths-ignore: []
"#;

        let parser = SerdeNorwayIgnorefileParser;
        let parsed = parse_ignorefile(&parser, input.as_bytes()).expect("parse Ignorefile YAML");

        assert_eq!(
            parsed,
            IgnoreConfig {
                gitignore_io: vec![],
                github: vec![],
                paths_ignore: vec![],
            }
        );
    }

    #[test]
    fn parses_ignorefile_without_paths_ignore() {
        let input = r#"
gitignore.io:
  - Ruby
github:
  - Python
"#;

        let parser = SerdeNorwayIgnorefileParser;
        let parsed = parse_ignorefile(&parser, input.as_bytes()).expect("parse Ignorefile YAML");

        assert_eq!(
            parsed,
            IgnoreConfig {
                gitignore_io: vec!["Ruby".to_string()],
                github: vec!["Python".to_string()],
                paths_ignore: vec![],
            }
        );
    }

    #[test]
    fn parses_empty_ignorefile() {
        let input = "";

        let parser = SerdeNorwayIgnorefileParser;
        let parsed = parse_ignorefile(&parser, input.as_bytes()).expect("parse empty Ignorefile");

        assert_eq!(
            parsed,
            IgnoreConfig {
                gitignore_io: vec![],
                github: vec![],
                paths_ignore: vec![],
            }
        );
    }
}
