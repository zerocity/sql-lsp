use crate::{response_types, utils, App};

pub struct LoginCommand {
    app: App
}

impl LoginCommand {
    pub fn new(app: App) -> Self {
        LoginCommand { app }
    }

    pub async fn login(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/login", &self.app.gitlab_url);

        let response = utils::client(url.as_str(), &self.app.gitlab_token)
            .send()
            .await?
            .json::<Vec<response_types::GitLabIssue>>()
            .await?;

        dbg!(response);

        Ok(())
    }
}