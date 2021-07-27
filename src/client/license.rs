use async_trait::async_trait;
use serde::Deserialize;

use super::{Client, Template};
use anyhow::Result;

#[derive(Deserialize)]
struct LicenseIndex {
    key: String,
    // name: String,
    // spdx_id: String,
    // url: String,
    // node_id: String,
}

#[derive(Deserialize)]
struct LicenseEntry {
    name: String,
    body: String,
    // key: String,
    // spdx_id: String,
    // url: String,
    // node_id: String,
    // html_url: String,
    // description: String,
    // implementation: String,
    // permissions: Vec<String>,
    // conditions: Vec<String>,
    // limitations: Vec<String>,
    // featured: bool,
}

pub struct LicenseClient {}

impl LicenseClient {
    pub fn new() -> Self {
        LicenseClient {}
    }
}

#[async_trait]
impl Client for LicenseClient {
    async fn list(&self) -> Result<Vec<String>> {
        let indice: Vec<LicenseIndex> = self.get("/licenses").await?;
        Ok(indice.iter().map(|li| li.key.clone()).collect())
    }

    async fn template(&self, name: &str) -> Result<Template> {
        let entry = self
            .get::<LicenseEntry>(&format!("/licenses/{}", name))
            .await?;
        Ok(Template {
            kind: String::from("license"),
            name: entry.name,
            body: entry.body,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_license_client_list() -> Result<()> {
        LicenseClient::new().list().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_license_client_template() -> Result<()> {
        LicenseClient::new().template("mit").await?;
        Ok(())
    }
}
