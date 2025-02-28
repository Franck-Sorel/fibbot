use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 3 {

        eprintln!("Usage: {} <input_string> <input_integer>", args[0]);
        std::process::exit(1);
 
    }

    // set the traceholders
    let input: u32 = args[2]
        .trim()
        .parse()
        .expect("Enter an integer as argument two");

    let text = read_pull_request::read_pull();
    
    let text = match text {
        Ok(string) => string,
        Err(_) => std::process::exit(1),
    };

    let text = text.as_str();
    let vector = extract_number::collect(text);

    for element in vector {
        println!("The fib_number is: {}", fib_number::fib_number(element))
    }

}

mod extract_number;
mod fib_number;
mod read_pull_request;