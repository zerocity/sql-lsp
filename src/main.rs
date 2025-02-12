use clap::{Parser, Subcommand};
use reqwest::{Client, RequestBuilder};

mod response_types;

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

fn client(url: &str, token: &str) -> RequestBuilder {
    let client = Client::new();
    client.get(url).bearer_auth(token)
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
            let response = client(
                "https://gitlab.com/api/v4/issues?assignee_id=4500276",
                &token,
            )
            .send()
            .await?
            .json::<Vec<response_types::GitLabIssue>>()
            .await?;
            dbg!(response);
        }
        None => (),
    }

    Ok(())
}
