use crate::ports::{GitignoreSource, GitignoreSourceError, GitignoreSourceResponse};
use reqwest::header::{ACCEPT, USER_AGENT};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ReqwestGitignoreSource {
    client: reqwest::Client,
    user_agent: String,
}

impl ReqwestGitignoreSource {
    pub fn new() -> Self {
        Self::default()
    }

    async fn get_text(&self, url: &str) -> Result<String, GitignoreSourceError> {
        let response = self
            .client
            .get(url)
            .header(USER_AGENT, &self.user_agent)
            .header(ACCEPT, "text/plain")
            .send()
            .await
            .map_err(|e| GitignoreSourceError::new(format!("{url}: {e}")))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| GitignoreSourceError::new(format!("{url}: {e}")))?;

        if status.is_success() {
            Ok(body)
        } else {
            let body = body.trim();
            if body.is_empty() {
                Err(GitignoreSourceError::new(format!("HTTP {status}: {url}")))
            } else {
                Err(GitignoreSourceError::new(format!(
                    "HTTP {status}: {url}: {body}"
                )))
            }
        }
    }
}

impl Default for ReqwestGitignoreSource {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("build reqwest client");

        Self {
            client,
            user_agent: "ignore-rs".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl GitignoreSource for ReqwestGitignoreSource {
    async fn fetch_gitignore_io(
        &self,
        target: &str,
    ) -> Result<GitignoreSourceResponse, GitignoreSourceError> {
        let url = format!("https://www.gitignore.io/api/{target}");
        let content = self.get_text(&url).await?;
        Ok(GitignoreSourceResponse { url, content })
    }

    async fn fetch_github(
        &self,
        target: &str,
    ) -> Result<GitignoreSourceResponse, GitignoreSourceError> {
        let url = format!(
            "https://raw.githubusercontent.com/github/gitignore/main/{target}.gitignore"
        );
        let content = self.get_text(&url).await?;
        Ok(GitignoreSourceResponse { url, content })
    }
}

