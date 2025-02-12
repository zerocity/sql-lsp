use clap::{Parser, Subcommand};

mod commands;
mod response_types;
mod utils;

// mod response_types;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Login,
    Project,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    //
    // from env
    //
    let token = std::env::var("GITLAB_TOKEN").expect("Could not find GITLAB_TOKEN in env");

    if let Some(cmd) = cli.command {
        match &cmd {
            Commands::Project => {
                let list = commands::projects::list_projects(&token).await?;
                dbg!(list);
                return Ok(());
            }
            Commands::Login => {
                let response = utils::client(
                    "https://gitlab.com/api/v4/issues?assignee_id=4500276",
                    &token,
                )
                .send()
                .await?
                .json::<Vec<response_types::GitLabIssue>>()
                .await?;
                dbg!(response);
                return Ok(());
            }
        }
    } else {
        println!("No command provided");
        return Ok(());
    }
}
