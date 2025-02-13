use clap::{Parser, Subcommand};
use edit;
use json_to_table::json_to_table;
use serde::Serialize;
use serde_json::json;
use crate::commands::RenderList;

pub mod commands;
pub mod response_types;
pub mod utils;


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
    Create,
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
    Tickets,
}

#[derive(Debug)]
pub struct App {
    pub gitlab_url: String,
    pub gitlab_token: String,
}

impl App {
    pub fn render(&self, to_be_rendered: impl Serialize, amount: usize) -> Result<String, serde_json::Error> {
        let table = json_to_table(&json!(&to_be_rendered)).collapse().to_string();
        let amount = format!(
            "\n-------------------------\nTotal Entries: {}",
            &amount
        );

        Ok(format!("{table}{amount}"))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    //
    // from env
    //
    let token = std::env::var("GITLAB_TOKEN").expect("Could not find GITLAB_TOKEN in env");
    let url = "https://gitlab.com/api/v4".to_string();

    let app = App {
        gitlab_url: url.clone(),
        gitlab_token: token.clone()
    };


    if let Some(cmd) = cli.command {
        match &cmd {
            Commands::Projects => {
                let project = commands::projects::ProjectsCommand::new(app);
                let project_list = project.list_projects().await?;

                let res = project.render_list(project_list)?;
                println!("{}", res);
                Ok(())
            }
            Commands::Create => {
                let git_url = utils::get_git_url()?;
                dbg!(git_url);
                //
                //
                //
                Ok(())
            }
            Commands::Login => {
                let login_command = commands::login::LoginCommand::new(app);
                login_command.login().await?;
                return Ok(());
            }
            Commands::CreateTicket {
                project_id,
                assignee_id,
                title,
                description,
            } => {
                let content = if title.is_none() || description.is_none() {
                    edit::edit(title.clone().unwrap_or_else(|| "".to_string()))
                        .unwrap_or_else(|_| String::new())
                } else {
                    String::new()
                };

                let (new_title, new_content) = commands::tickets::extract_title_and_content(content)?;

                let issue_command = commands::tickets::TicketsCommand::new(app);
                issue_command.create_issue(
                    *project_id,
                    commands::tickets::CreateIssue {
                        assignee_id: *assignee_id,
                        title: new_title.to_string(),
                        description: new_content.clone(),
                    }
                ).await?;

                Ok(())
            }
            Commands::Tickets => {
                let issue_command = commands::tickets::TicketsCommand::new(app);
                issue_command.list_issues().await?;
                Ok(())
            }
        }
    } else {
        println!("No command provided");
        return Ok(());
    }
}