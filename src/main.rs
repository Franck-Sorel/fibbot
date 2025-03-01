use add_comment::post_comment;
use fib_number::fib_number;
use std::env;



#[tokio::main]
async fn main() {

    let boool = true;
    if !boool {
        return;
    }
    let pr_number = 1 as u128;

    // let pr_number = env::var("PR_NUMBER")
    // .expect("PR_NUMBER not set")
    // .parse::<u128>()
    // .expect("Invalid PR_NUMBER");

    println!("PR_number: {:?}", pr_number);
    
    let limit = env::var("TRESHOLD")
    .expect("THRESHOLD not set")
    .parse::<u128>()
    .expect("Invalid THRESHOLD");

    // Pull request content as String    

   let content = get_from_pull_request::get_pull_request(pr_number, limit).await;

   for i in &content {
    println!("Fibonacci {}, is : {}", i, fib_number::fib_number(*i));
   }
   let mut response =
        String::from("## YOUR FIBONACCI :\n");
    for &num in &content {
        let fib = fib_number(num);
        response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
    }

   if let Err(e) = post_comment(&response).await {
    eprintln!("Error posting comment: {}", e);
}
}

mod extract_number;
mod fib_number;
mod get_from_pull_request;
mod add_comment;
