use crate::extract_number::collect;

pub async fn get_pull_request(pr_number: u128, limit: u128) -> Vec<u128> {

    let files = octocrab::instance().pulls("Nkwenti-severian-Ndongtsop", "Fibonacci-bot").list_files(pr_number.try_into().unwrap()).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let nums = collect(&files.as_str(), limit);
    println!("Collected Nums: {nums:?}");
    nums
    
 }