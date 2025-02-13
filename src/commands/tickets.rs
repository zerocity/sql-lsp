use json_to_table::json_to_table;
use serde::Serialize;
use serde_json::json;
use crate::{response_types, utils, App};

pub struct TicketsCommand {
    app: App
}

#[derive(Serialize)]
pub(crate) struct CreateIssue {
    pub title: String,
    pub description: Option<String>,
    pub assignee_id: u64,
}

impl TicketsCommand {
    pub fn new(app: App) -> TicketsCommand {
        TicketsCommand { app }
    }

    pub async fn list_issues(
        &self
    ) -> Result<Vec<response_types::GitLabIssue>, Box<dyn std::error::Error>> {
        let url = format!("{}/issues?membership=true", &self.app.gitlab_url);

        // This is a placeholder for the code that will be generated
        let res = utils::client(url.as_str(), &self.app.gitlab_token)
            .send()
            .await?
            .json::<Vec<response_types::GitLabIssue>>()
            .await?;

        let a = json_to_table(&json!(&res)).to_string();

        print!("{}", a);

        return Ok(res);
    }

    pub async fn create_issue(
        &self,
        project_id: u64,
        create_issue: CreateIssue,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}/issues", &self.app.gitlab_url, project_id);

        let res = utils::client_post(url.as_str(), &self.app.gitlab_token)
            .json(
                &create_issue
            )
            .send()
            .await?;

        dbg!(&res);

        Ok(())
    }
}