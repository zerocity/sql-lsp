use clap::{Parser, Subcommand};
use gitlab::{
    api::{
        self,
        users::{CurrentUser, UserProjects, Users},
        Query,
    },
    Gitlab,
};

use serde::Deserialize;

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

// The return type of a `Project`. Note that GitLab may contain more information, but you can
// define your structure to only fetch what is needed.
#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Debug, Deserialize)]
struct UserProject {
    name: String,
}

fn main() {
    let cli = Cli::parse();

    //
    // from env
    //
    let token = std::env::var("GITLAB_TOKEN").expect("Could not find GITLAB_TOKEN in env");

    match &cli.command {
        Some(Commands::Login) => {
            let client = Gitlab::new("gitlab.com", token).expect("Gitlab client");
            let user: User = CurrentUser::builder()
                .build()
                .unwrap()
                .query(&client)
                .unwrap();

            let user_project: Vec<UserProject> = UserProjects::builder()
                .user(user.id)
                .build()
                .unwrap()
                .query(&client)
                .unwrap();

            // let raw_data: Vec<u8> = api::raw(&b).query(&client).unwrap();
            // let raw = String::from_utf8(raw_data).unwrap(); // raw_data.into();

            dbg!(&user);
            dbg!(&user_project);
        }
        None => (),
    }
}
