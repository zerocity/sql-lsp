use serde::{Deserialize, Serialize};

/// GitLab User Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabUser {
    pub id: u64,
    pub username: String,
    pub name: String,
    pub state: String,
    pub avatar_url: Option<String>,
    pub web_url: String,
}

/// GitLab Project Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabProject {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub web_url: String,
    pub visibility: String,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub owner: Option<GitLabUser>,
    pub star_count: u64,
    pub forks_count: u64,
}

/// GitLab Issue Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabIssue {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub closed_by: Option<GitLabUser>,
    pub labels: Vec<String>,
    pub assignee: Option<GitLabUser>,
    pub assignees: Vec<GitLabUser>,
    pub author: GitLabUser,
    pub web_url: String,
}

/// GitLab Merge Request Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabMergeRequest {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub merged_by: Option<GitLabUser>,
    pub source_branch: String,
    pub target_branch: String,
    pub labels: Vec<String>,
    pub author: GitLabUser,
    pub assignee: Option<GitLabUser>,
    pub assignees: Vec<GitLabUser>,
    pub reviewers: Vec<GitLabUser>,
    pub web_url: String,
}

/// GitLab Commit Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabCommit {
    pub id: String,
    pub short_id: String,
    pub title: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_date: String,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_date: String,
    pub web_url: String,
}

/// GitLab Pipeline Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabPipeline {
    pub id: u64,
    pub project_id: u64,
    pub status: String,
    pub ref_name: String,
    pub sha: String,
    pub web_url: String,
    pub created_at: String,
    pub updated_at: String,
}

/// GitLab CI/CD Job Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabJob {
    pub id: u64,
    pub name: String,
    pub stage: String,
    pub status: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub duration: Option<f64>,
    pub web_url: String,
}

/// GitLab Repository Branch Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabBranch {
    pub name: String,
    pub merged: bool,
    pub protected: bool,
    pub developers_can_push: bool,
    pub developers_can_merge: bool,
    pub web_url: String,
}

/// GitLab Repository Tag Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabTag {
    pub name: String,
    pub message: Option<String>,
    pub target: String,
}

/// GitLab Repository File Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabFile {
    pub file_name: String,
    pub file_path: String,
    pub size: u64,
    pub encoding: String,
    pub content: String,
    pub content_sha256: Option<String>,
    pub ref_name: String,
    pub blob_id: String,
    pub commit_id: String,
    pub last_commit_id: String,
}

/// GitLab Group Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabGroup {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub visibility: String,
    pub web_url: String,
}

/// GitLab Deployment Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabDeployment {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// GitLab Release Response
#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabRelease {
    pub tag_name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub released_at: String,
    pub author: GitLabUser,
}
