use async_trait::async_trait;
use serde::Deserialize;

use super::{Client, Template};
use anyhow::Result;

#[derive(Deserialize)]
struct GitignoreEntry {
    name: String,
    source: String,
}

pub struct GitignoreClient {}

impl GitignoreClient {
    pub fn new() -> Self {
        GitignoreClient {}
    }
}

#[async_trait]
impl Client for GitignoreClient {
    async fn list(&self) -> Result<Vec<String>> {
        Ok(self.get("/gitignore/templates").await?)
    }

    async fn template(&self, name: &str) -> Result<Template> {
        let entry = self
            .get::<GitignoreEntry>(&format!("/gitignore/templates/{}", name))
            .await?;
        Ok(Template {
            kind: String::from("gitignore"),
            name: entry.name,
            body: entry.source,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_gitignore_client_list() -> Result<()> {
        GitignoreClient::new().list().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_gitignore_client_template() -> Result<()> {
        GitignoreClient::new().template("Python").await?;
        Ok(())
    }
}
