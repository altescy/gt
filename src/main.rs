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
    #[error("TargetNotFound: {0}")]
    TargetNotFound(String),
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = clap_app!(gt =>
        (version: "1.0.0")
        (author: "altescy <altescy@fastmail.com>")
        (about: "Generate template of gitignore / license")
        (@arg type: -t --type +takes_value "specify target type")
        (@subcommand generate =>
            (about: "generate template of gitignore / license")
            (@arg NAME: +required "Sets the name of template")
        )
        (@subcommand list =>
            (about: "show the list of available gitignore / license names")

        )
    )
    .get_matches();

    let mut use_gitignore = false;
    let mut use_license = false;

    if let Some(targets) = matches.value_of("type") {
        for target in targets.split(",") {
            match target {
                "gitignore" => {
                    use_gitignore = true;
                }
                "license" => {
                    use_license = true;
                }
                _ => Err(GtError::TargetNotFound(String::from(target)))?,
            }
        }
    } else {
        use_gitignore = true;
        use_license = true;
    }

    let c = client::UnifiedClient::new(use_gitignore, use_license);

    if let Some(matches) = matches.subcommand_matches("generate") {
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
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        for name in c.list().await? {
            println!("{}", &name);
        }
    }

    Ok(())
}
