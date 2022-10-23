use snyk::client::{RestClient};
use std::fs;

#[tokio::test]
async fn get_test() {
    let snyk_token =  fs::read_to_string(".secret").unwrap();
    let client = RestClient { token:snyk_token};
    let out = client.get_me().await.unwrap();
    let orgs = out.orgs.iter();

    let mut orglist: Vec<String> = Vec::new();

    for org in orgs {
        orglist.push(org.name.to_string());
    };

    println!("Orgs:\n\n{:#?}", orglist);
}