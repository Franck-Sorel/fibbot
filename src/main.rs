
fn main() {

    let boool = true;
    if !boool {
        return;
    }

    let input = 100;

    // Pull request content as String
    let text = read_pull_request::read_pull();

    let text = match text {
        Ok(string) => string,
        Err(_) => { eprintln!("Unable to read the pull request content as string");
        std::process::exit(1); }
    };

    let text = text.as_str();
    let vector = extract_number::collect(text, input);

    for element in vector {
        println!("The fib_number is: {}", fib_number::fib_number(element));
    }
}

mod extract_number;
mod fib_number;
mod read_pull_request;
