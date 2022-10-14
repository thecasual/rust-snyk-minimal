
use reqwest;
use serde_json::{self, Value};
use std::collections::HashMap;

static SNYK_BASE_URL: &str = "https://api.snyk.io/api/v1";

pub struct RestClient {
    pub token: String
}

struct Header { 
    
}

impl RestClient {
    pub async fn get_me(self){
        let url = format!("{}/user/me", SNYK_BASE_URL);
        let client = reqwest::Client::new();
        let res = client.get(url)
        .header("Authorization", format!("token {}", self.token))
        .send()
        .await
        .unwrap()
        .text()
        .await;
        
    }

    pub fn new(token: String) -> Self {
         Self { token } 
    }
}