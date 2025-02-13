use serde::{Serialize, Deserialize};
use crate::{response_types, utils, App};
use serde_json;
use serde_json::{ Error};
use crate::commands::RenderList;

pub struct ProjectsCommand {
    app: App
}

#[derive(Serialize, Deserialize)]
pub struct ProjectsList {
    id: u64,
    name: String,
}

impl ProjectsCommand {
    pub fn new(app: App) -> Self {
        Self {
            app
        }
    }

    pub async fn list_projects(
        &self,
    ) -> Result<Vec<response_types::GitLabProject>, Box<dyn std::error::Error>> {
        let url = format!("{}/projects?membership=true", &self.app.gitlab_url);

        // This is a placeholder for the code that will be generated
        let res = utils::client(url.as_str(), &self.app.gitlab_token)
            .send()
            .await?
            .json::<Vec<response_types::GitLabProject>>()
            .await?;

        Ok(res)
    }
}

impl RenderList<Vec<response_types::GitLabProject>> for ProjectsCommand {

    fn render_list(&self, project_list: Vec<response_types::GitLabProject>) -> Result<String, Error> {
        let project_list_new: Vec<ProjectsList> = project_list.iter().map(|p| {
            ProjectsList {
                id: p.id,
                name: p.name.clone(),
            }
        }).collect();

        self.app.render(project_list_new, project_list.len())
    }
}
