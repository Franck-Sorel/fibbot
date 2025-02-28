
fn main() {

    let boool = true;
    if !boool {
        return;
    }

    let limit = 100 as u128;

    // Pull request content as String
    let text = read_pull_request::read_pull();
    let content = text.expect("error taking the pull request content as sting");
    
    

    let content = content.as_str();
    let vector = extract_number::collect(content, limit);

    for element in vector {
        println!("The fib_number is: {}", fib_number::fib_number(element));
    }
}

mod extract_number;
mod fib_number;
mod read_pull_request;
mod get_from_pull_request;