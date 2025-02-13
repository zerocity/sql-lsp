use json_to_table::json_to_table;
use serde::Serialize;
use serde_json::{json, Error};
use crate::{response_types, utils, App};
use crate::commands::RenderList;

pub struct TicketsCommand {
    app: App
}

#[derive(Serialize)]
pub struct CreateIssue {
    pub title: String,
    pub description: Option<String>,
    pub assignee_id: u64,
}

#[derive(Serialize)]
pub struct TicketList {
    id: u64,
    title: String,
    description: Option<String>
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

impl RenderList<Vec<response_types::GitLabIssue>> for TicketsCommand {
    fn render_list(&self, input: Vec<response_types::GitLabIssue>) -> Result<String, Error> {
        let ticket_list: Vec<TicketList> = input.iter().map(|p| {
            TicketList {
                id: p.id,
                title: p.title.clone(),
                description: p.description.clone()
            }
        }).collect();

        self.app.render(ticket_list, input.len())
    }
}


pub fn extract_title_and_content(
    content: String,
) -> Result<(String, Option<String>), Box<dyn std::error::Error>> {
    if content.is_empty() {
        return Err("Title and content cannot be parsed".into());
    }

    if let Some((title, content)) = content.split_once("\n\n") {
        return Ok((
            title.to_string(),
            Some(content.to_string().trim().to_string()),
        ));
    }

    Ok((content, None))
}



#[cfg(test)]
mod tests {
    const TITLE_AND_CONTENT_ERROR: &'static str = "Title and content cannot be parsed";
    use super::*;
    #[test]
    fn throw_error_when_content_was_not_provided() {
        let a = extract_title_and_content("".to_string())
            .unwrap_err()
            .to_string();
        assert_eq!(a, TITLE_AND_CONTENT_ERROR);
    }
    #[test]
    fn extract_title() {
        let a = extract_title_and_content("Demo ticket title".to_string()).unwrap();
        assert_eq!(a.0, "Demo ticket title");
    }
    #[test]
    fn extract_content() {
        let (_title, content) =
            extract_title_and_content("Demo ticket title\n\nsomething".to_string()).unwrap();
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
