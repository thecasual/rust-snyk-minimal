
use reqwest;
use serde_json::{self, Value};
//use serde_json::{self, Value, json};

static SNYK_BASE_URL: &str = "https://api.snyk.io/api/v1";

pub struct RestClient {
    pub token: String
}

pub struct SnykOrganzations {
    organizations: [String]
}

impl RestClient {
    pub async fn get_me(&self) -> Value {
        let url = format!("{}/user/me", SNYK_BASE_URL);
        let client = reqwest::Client::new();
        let res = client.get(url)
        .header("Authorization", format!("token {}", self.token))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

        res

        //let orgs = SnykOrganzations{ res["orgs"]};
        //orgs
        
    }
}