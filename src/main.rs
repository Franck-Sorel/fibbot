fn main() {

    let boool = true;
    if !boool {
        return;
    }

    let limit = 100 as u128;

    // Pull request content as String
    let text = read_pull_request::read_pull();
    let content = text.expect("error taking the pull request content as sting");
    
    let pr_number = 1 as u128;

    let _content = content.as_str();

   let _content = get_from_pull_request::get_pull_request(pr_number, limit);

}

mod extract_number;
mod fib_number;
mod read_pull_request;
mod get_from_pull_request;