use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use serde::de::DeserializeOwned;

pub mod gitignore;
pub mod license;

pub use gitignore::GitignoreClient;
pub use license::LicenseClient;

const ENDPOINT_URL: &str = "https://api.github.com";
const USER_AGENT: &str = "com.github.altescy.gt";

pub struct Template {
    pub kind: String,
    pub name: String,
    pub body: String,
}

#[async_trait]
pub trait Client {
    async fn get<T: DeserializeOwned>(&self, uri: &str) -> Result<T> {
        Ok(reqwest::Client::new()
            .get(format!("{}{}", ENDPOINT_URL, uri))
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .header(reqwest::header::ACCEPT, "application/vnd.github.v3+json")
            .send()
            .await?
            .json::<T>()
            .await?)
    }
    async fn find(&self, query: &str) -> Result<Option<Template>> {
        let query = query.trim();
        let names = self.list().await?;
        if let Some(position) = names
            .iter()
            .position(|name| name.to_ascii_lowercase() == query.to_ascii_lowercase())
        {
            let name = names[position].clone();
            Ok(Some(self.template(&name).await?))
        } else {
            Ok(None)
        }
    }
    async fn list(&self) -> Result<Vec<String>>;
    async fn template(&self, name: &str) -> Result<Template>;
}

pub struct UnifiedClient {
    pub gitignore: Option<GitignoreClient>,
    pub license: Option<LicenseClient>,
}

impl UnifiedClient {
    pub fn new(gitignore: bool, license: bool) -> Self {
        UnifiedClient {
            gitignore: if gitignore {
                Some(GitignoreClient::new())
            } else {
                None
            },
            license: if license {
                Some(LicenseClient::new())
            } else {
                None
            },
        }
    }
}

#[async_trait]
impl Client for UnifiedClient {
    async fn list(&self) -> Result<Vec<String>> {
        let mut names = vec![];
        if let Some(c) = &self.gitignore {
            names.extend(c.list().await?)
        }
        if let Some(c) = &self.license {
            names.extend(c.list().await?)
        }
        Ok(names)
    }
    async fn template(&self, name: &str) -> Result<Template> {
        if let Some(c) = &self.gitignore {
            Ok(c.template(name).await?)
        } else if let Some(c) = &self.license {
            Ok(c.template(name).await?)
        } else {
            panic!("invalid name")
        }
    }
    async fn find(&self, query: &str) -> Result<Option<Template>> {
        let template = if let Some(c) = &self.gitignore {
            c.find(query).await?
        } else {
            None
        };

        if let Some(_) = template {
            return Ok(template);
        }

        let template = if let Some(c) = &self.license {
            c.find(query).await?
        } else {
            None
        };

        Ok(template)
    }
}
