use reqwest::{Client, RequestBuilder};

pub fn client(url: &str, token: &str) -> RequestBuilder {
    let client = Client::new();
    client.get(url).bearer_auth(token)
}
