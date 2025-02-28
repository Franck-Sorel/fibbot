use std::env;

use reqwest::blocking::Client;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct PullRequest {
    body: String,

}


pub fn read_pull() -> Result<String, reqwest::Error> {
    let client = Client::new();

    let pull_request_number = 1;

    let owner_repo = env::var("GITHUB_REPOSITORY").expect("Bad Repo");
    let pull_request_url = format!("https://api.github.com/repos/{owner_repo}/pulls/{pull_request_number}");

    let token = match env::var("GH_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("GitHub token not found in environment variables.");
            std::process::exit(1);
        }
        
    };

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