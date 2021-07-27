#[macro_use]
extern crate clap;

use anyhow::Result;
use thiserror::Error;

mod client;
use client::Client;

#[derive(Debug, Error)]
enum GtError {
    #[error("TemplateNotFound: {0}")]
    TemplateNotFound(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = clap_app!(gt =>
        (version: "1.0")
        (author: "altescy <altescy@fastmail.com>")
        (about: "Generate template of gitignore / license")
        (@arg NAME: +required "Sets the name of template")
    )
    .get_matches();

    let c = client::UnifiedClient::new(true, true);

    let names = matches.value_of("NAME").unwrap();
    for name in names.split(",") {
        if let Some(template) = c.find(name).await? {
            match template.kind.as_str() {
                "gitignore" => {
                    println!("##");
                    println!("##  {}", &template.name);
                    println!("##");
                    println!("");
                    println!("{}", &template.body);
                }
                "license" => {
                    println!("{}", &template.body);
                }
                _ => (),
            }
        } else {
            Err(GtError::TemplateNotFound(String::from(name)))?;
        }
    }

    Ok(())
}
