use reqwest::Client;
use std::env;

pub async fn post_comment(pr_content: &str) -> Result<(), reqwest::Error> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u32>()
        .expect("Invalid PR_NUMBER");
    println!("PR_NUMBER: {:?}", pr_number);
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    println!("GITHUB_TOKEN: {:?}", github_token);
    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );

    println!("URL: {:?}", url);
    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .json(&serde_json::json!({ "body": pr_content }))
        .send()
        .await?;
    println!("RESPONSE: {:?}", response);

    if response.status().is_success() {
        println!("✅ Comment posted successfully.");
    } else {
        eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
    }

    Ok(())
}
