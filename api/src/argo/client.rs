use reqwest::Client;

use super::urls::*;

pub struct ArgoClient {
    token: String,
    url: String,
    namespace: String,
    client: Client,
}

impl ArgoClient {
    pub fn new(token: String, url: String, namespace: String) -> ArgoClient {
        ArgoClient {
            token,
            url,
            namespace,
            client: reqwest::Client::new()
        }
    }
}