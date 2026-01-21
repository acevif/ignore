#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitignoreSourceError {
    message: String,
}

impl GitignoreSourceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for GitignoreSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gitignore source error: {}", self.message)
    }
}

impl std::error::Error for GitignoreSourceError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitignoreSourceResponse {
    pub url: String,
    pub content: String,
}

#[async_trait::async_trait]
pub trait GitignoreSource {
    async fn fetch_gitignore_io(
        &self,
        target: &str,
    ) -> Result<GitignoreSourceResponse, GitignoreSourceError>;

    async fn fetch_github(
        &self,
        target: &str,
    ) -> Result<GitignoreSourceResponse, GitignoreSourceError>;
}
