use std::env;

use reqwest::{blocking::Client, Error};
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct PullRequest {
    title: String,
    body: String,

}

// fn get_pull_request_content(repo: &str, pr_number: u32) -> Result<String, reqwest::Error> {
//     let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
//     let url = format!("https://api.github.com/repos/{}/pulls/{}", repo, pr_number);

//     let client = Client::new();
//     let response = client.get(&url)
//         .header("Authorization", format!("token {}", token))
//         .send()?;

//     if response.status().is_success() {
//         let pr: PullRequest = response.json()?;
//         Ok(pr.body)
//     } else {
//         Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
//     }
// }

pub fn read_pull() -> Result<String, reqwest::Error> {
    let client = Client::new();

    let pull_request_number = env::var("PR_NUMBER").expect("Bad pull request number").parse::<u32>().expect("Bad_parsing pull request number");
    let owner_repo = env::var("GITHUB_REPOSITORY").expect("Bad Repo");
    let pull_request_url = format!("https://api.github.com/repos/{owner_repo}/pulls/{pull_request_number}");

    let token = env::var("GITHUB_TOKEN").expect("token_Error");

    let response = client.get(&pull_request_url)
        .header("User-Agent", "foo")
        .header("Authorization", format!("token {}", token))
        .header("Accept", "application/vnd.github+json")
        .send()?;

    
        if response.status().is_success() {
            let pr: PullRequest = response.json()?;
            Ok(pr.body)
        } else {
            Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
        }
    

    
    
}