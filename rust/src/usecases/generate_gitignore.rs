use crate::domain::Ignorefile;
use crate::ports::GitignoreSource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateGitignoreError {
    message: String,
}

impl GenerateGitignoreError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GenerateGitignoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "generate gitignore error: {}", self.message)
    }
}

impl std::error::Error for GenerateGitignoreError {}

fn push_blank_lines(output: &mut String, count: usize) {
    for _ in 0..count {
        output.push('\n');
    }
}

pub async fn generate_gitignore<S: GitignoreSource>(
    config: &Ignorefile,
    source: &S,
) -> Result<String, GenerateGitignoreError> {
    let mut output = String::new();

    output.push_str("# This .gitignore file is auto-generated. Do not edit!\n");
    output.push_str("# Edit Ignorefile instead.\n");
    push_blank_lines(&mut output, 2);

    let gitignore_io_futures = config.gitignore_io.iter().map(|target| async move {
        source
            .fetch_gitignore_io(target)
            .await
            .map_err(|e| GenerateGitignoreError::new(format!("gitignore.io {target}: {e}")))
    });
    let gitignore_io_entries = futures::future::try_join_all(gitignore_io_futures).await?;
    for fetched in gitignore_io_entries {
        output.push_str(&fetched.content);
        push_blank_lines(&mut output, 3);
    }

    let github_futures = config.github.iter().map(|target| async move {
        source
            .fetch_github(target)
            .await
            .map_err(|e| GenerateGitignoreError::new(format!("github {target}: {e}")))
    });
    let github_entries = futures::future::try_join_all(github_futures).await?;
    for fetched in github_entries {
        output.push_str(&format!("# Downloaded from {}\n\n", fetched.url));
        output.push_str(&fetched.content);
        push_blank_lines(&mut output, 3);
    }

    output.push_str("### Project specific settings ###\n\n");

    for entry in &config.paths_ignore {
        output.push_str(entry);
        output.push('\n');
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{GitignoreSource, GitignoreSourceError, GitignoreSourceResponse};

    #[derive(Debug, Default)]
    struct FakeGitignoreSource;

    #[async_trait::async_trait]
    impl GitignoreSource for FakeGitignoreSource {
        async fn fetch_gitignore_io(
            &self,
            target: &str,
        ) -> Result<GitignoreSourceResponse, GitignoreSourceError> {
            match target {
                "Ruby" => Ok(GitignoreSourceResponse {
                    url: "https://www.gitignore.io/api/Ruby".to_string(),
                    content: "# Ruby gitignore\n".to_string(),
                }),
                "direnv" => Ok(GitignoreSourceResponse {
                    url: "https://www.gitignore.io/api/direnv".to_string(),
                    content: "# direnv gitignore\n".to_string(),
                }),
                other => Err(GitignoreSourceError::new(format!(
                    "unexpected gitignore.io target: {other}"
                ))),
            }
        }

        async fn fetch_github(
            &self,
            target: &str,
        ) -> Result<GitignoreSourceResponse, GitignoreSourceError> {
            match target {
                "Python" => Ok(GitignoreSourceResponse {
                    url: "https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore"
                        .to_string(),
                    content: "# Python gitignore\n".to_string(),
                }),
                other => Err(GitignoreSourceError::new(format!(
                    "unexpected github target: {other}"
                ))),
            }
        }
    }

    #[tokio::test]
    async fn generates_gitignore_without_network_access() {
        let config = Ignorefile {
            gitignore_io: vec!["Ruby".to_string(), "direnv".to_string()],
            github: vec!["Python".to_string()],
            paths_ignore: vec!["foo".to_string(), "bar".to_string(), "!baz".to_string()],
        };

        let source = FakeGitignoreSource;
        let generated = generate_gitignore(&config, &source)
            .await
            .expect("generate .gitignore");

        let expected = "\
# This .gitignore file is auto-generated. Do not edit!\n\
# Edit Ignorefile instead.\n\
\n\
\n\
# Ruby gitignore\n\
\n\
\n\
\n\
# direnv gitignore\n\
\n\
\n\
\n\
# Downloaded from https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore\n\
\n\
# Python gitignore\n\
\n\
\n\
\n\
### Project specific settings ###\n\
\n\
foo\n\
bar\n\
!baz\n";

        assert_eq!(generated, expected);
    }
}
