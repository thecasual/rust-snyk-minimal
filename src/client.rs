use reqwest;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

static SNYK_BASE_URL: &str = "https://api.snyk.io/api/v1";

pub struct RestClient {
    pub token: String
}

pub struct SnykOrganzations {
    organizations: [String]
}

impl RestClient {
    pub async fn get_me(&self) -> Result<SnykMe, reqwest::Error> {
        let url = format!("{}/user/me", SNYK_BASE_URL);
        let client = reqwest::Client::new();
        let res: _ = client.get(url)
        .header("Authorization", format!("token {}", self.token))
        .send()
        .await
        .unwrap()
        .json::<SnykMe>()
        .await;
        res
        
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnykMe {
    pub email: Value,
    pub id: String,
    pub orgs: Vec<Org>,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    pub group: Group,
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
}
