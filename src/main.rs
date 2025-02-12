use std::error::Error;
use std::ops::Deref;
use clap::{Parser, Subcommand};
use edit;

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
    Projects,
    CreateTicket {
        #[arg(short, long)]
        project_id: u64,
        #[arg(short, long)]
        assignee_id: u64,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Tickets
}

const URL: &'static str = "https://gitlab.com/api/v4/issues?assignee_id=4500276";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    //
    // from env
    //
    let token = std::env::var("GITLAB_TOKEN").expect("Could not find GITLAB_TOKEN in env");

    if let Some(cmd) = cli.command {
        match &cmd {
            Commands::Projects => {
                let list = commands::projects::list_projects(&token).await?;
                dbg!(list);
                return Ok(());
            }
            Commands::Login => {
                let response = utils::client(
                    URL,
                    &token,
                )
                .send()
                .await?
                .json::<Vec<response_types::GitLabIssue>>()
                .await?;
                dbg!(response);
                return Ok(());
            }
            Commands::CreateTicket { project_id, assignee_id, title, description } => {
                let content = if title.is_none() || description.is_none() {
                    edit::edit("Demo ticket title").unwrap_or_else(|_| String::new())
                } else { String::new() };

                let (new_title, new_content) = extract_title_and_content(content)?;

                commands::projects::create_issue(&token, *project_id, *assignee_id, new_title.to_string(), new_content.clone()).await?;
                Ok(())
            },
            Commands::Tickets => {
                commands::projects::list_issues(&token).await?;
                Ok(())
            }
        }
    } else {
        println!("No command provided");
        return Ok(());
    }
}

fn extract_title_and_content(content: String) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    if content.is_empty() {
        return Err("Title and content cannot be parsed".into());
    }

    if let Some((title, content)) = content.split_once("\n\n") {
        return Ok((title.to_string(), Some(content.to_string().trim().to_string())))
    }

    Ok((content, None))
}

const TITLE_AND_CONTENT_ERROR: &'static str = "Title and content cannot be parsed";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn throw_error_when_content_was_not_provided() {
        let a = extract_title_and_content("".to_string()).unwrap_err().to_string();
        assert_eq!(a, TITLE_AND_CONTENT_ERROR);
    }
    #[test]
    fn extract_title() {
        let a = extract_title_and_content("Demo ticket title".to_string()).unwrap();
        assert_eq!(a.0, "Demo ticket title");
    }
    #[test]
    fn extract_content() {
        let (_title, content) = extract_title_and_content("Demo ticket title\n\nsomething".to_string()).unwrap();
        assert_eq!(content.unwrap(), "something");
    }

    #[test]
    fn extract_title_and_content_test() {
        let a = extract_title_and_content("Demo ticket title\n\nsomething".to_string()).unwrap();

        assert_eq!(a.0, "Demo ticket title");
        assert_eq!(a.1.unwrap(), "something");
    }

    #[test]
    fn extract_title_and_content_test_with_three_newlines() {
        let a = extract_title_and_content("Demo ticket title\n\n\nsomething".to_string()).unwrap();

        assert_eq!(a.0, "Demo ticket title");
        assert_eq!(a.1.unwrap(), "something");
    }
}