use anyhow::{anyhow, Context, Result};

use git_url_parse::GitUrl;
use reqwest::{Client, RequestBuilder};

pub fn client(url: &str, token: &str) -> RequestBuilder {
    let client = Client::new();
    client.get(url).bearer_auth(token)
}
pub fn client_post(url: &str, token: &str) -> RequestBuilder {
    let client = Client::new();
    client
        .post(url)
        .bearer_auth(token)
        .header("Content-Type", "application/json")
}

pub fn get_git_url() -> Result<GitUrl> {
    let git_config_file = gix_config::File::from_git_dir(".git".into())?;
    let origin_url = git_config_file
        .path("remote.origin.url")
        .ok_or(anyhow!("Could not found remote url"))?;

    let origin_url = origin_url.to_string();
    GitUrl::parse(&origin_url).with_context(|| format!("Could not parse git url: {}", origin_url))
}
