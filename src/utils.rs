use reqwest::{Client, RequestBuilder};

pub fn client(url: &str, token: &str) -> RequestBuilder {
    let client = Client::new();
    client.get(url).bearer_auth(token)
}
pub fn client_post(url: &str, token: &str) -> RequestBuilder {
    let client = Client::new();
    client.post(url).bearer_auth(token).header("Content-Type", "application/json")
}
