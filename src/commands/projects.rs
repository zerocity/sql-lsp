use crate::{response_types, utils};

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
