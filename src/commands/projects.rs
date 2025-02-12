use serde::Serialize;
use crate::{response_types, utils};
use json_to_table::json_to_table;
use serde_json;
use serde_json::json;

pub async fn list_projects(
    token: &str,
) -> Result<Vec<response_types::GitLabProject>, Box<dyn std::error::Error>> {
    // This is a placeholder for the code that will be generated
    let res = utils::client("https://gitlab.com/api/v4/projects?membership=true", &token)
        .send()
        .await?
        .json::<Vec<response_types::GitLabProject>>()
        .await?;

    for project in &res {
        println!("{}: {}", project.id, project.name);
    }
    println!(
        "\n-------------------------\nTotal projects: {}",
        &res.len()
    );
    return Ok(res);
}

pub async fn list_issues(
    token: &str,
) -> Result<Vec<response_types::GitLabIssue>, Box<dyn std::error::Error>> {
    // This is a placeholder for the code that will be generated
    let res = utils::client("https://gitlab.com/api/v4/issues?membership=true", &token)
        .send()
        .await?
        .json::<Vec<response_types::GitLabIssue>>()
        .await?;

        let a = json_to_table(&json!(&res)).to_string();

        print!("{}", a);

    return Ok(res);
}

#[derive(Serialize)]
struct CreateIssue {
    pub title: String,
    pub description: Option<String>,
    pub assignee_id: u64,
}

pub async fn create_issue(
    token: &str,
    project_id: u64,
    assignee_id: u64,
    title: String,
    description: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = utils::client_post(format!("https://gitlab.com/api/v4/projects/{project_id}/issues").as_str(), &token)
        .json(
            &CreateIssue {
                title,
                description,
                assignee_id,
            }
        )
        .send()
        .await?;

    dbg!(&res);

    Ok(())
}