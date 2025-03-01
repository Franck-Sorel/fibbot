use std::env;

use reqwest::Client;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct PullRequest {
    title: String,
    body: String,

}


pub async fn read_pull() -> Result<String, reqwest::Error> {
    let client = Client::new();

    let pull_request_number = 1;

    let owner_repo = env::var("GITHUB_REPOSITORY").expect("Bad Repo");
    let pull_request_url = format!("https://api.github.com/repos/{owner_repo}/pulls/{pull_request_number}/files");

    let token = match env::var("GH_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("GitHub token not found in environment variables.");
            std::process::exit(1);
        }
        
    };

    let response = client
        .get(&pull_request_url)
        .header("Authorization", format!("token {}", token))
        .header("Accept", "application/vnd.github.v3+json")
        .send().await?
        .json::<PullRequest>().await?;
    Ok(format!("Title: {}\nBody: {}", response.title, response.body))
    
    

    
    
}
