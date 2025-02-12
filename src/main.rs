use clap::{Parser, Subcommand};
use serde_json;

mod project;
use project::Project;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Login,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    //
    // from env
    //
    let token = std::env::var("GITLAB_TOKEN").expect("Could not find GITLAB_TOKEN in env");

    match &cli.command {
        Some(Commands::Login) => {
            let client = reqwest::Client::new();
            let result = client
                .get("https://gitlab.com/api/v4/projects")
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await?
                .text()
                .await?;

            // dbg!(&result);

            let a = serde_json::from_str::<Vec<Project>>(&result);
            match a {
                Ok(a) => {
                    println!("Length --->{:#?}", a.len());
                }
                Err(e) => {
                    println!("--->{:#?}", e);
                }
            }

            Ok(())
        }
        None => Ok(()),
    }

    // Continued program logic goes here...
}
