use snyk::client::{RestClient};
use tokio::test;
use std::env;
use std::fs;

#[tokio::test]
async fn get_test() {
    let snykToken =  fs::read_to_string(".secret").unwrap();
    let client = RestClient { token:snykToken};
    client.get_me().await;
    print!("Complete");
}