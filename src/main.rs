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

    let text = "pull_request 123 since 1990-12-03 ";

    let vector = extract_number::collect(text);

    for element in vector {
        println!("The fib_number is: {}", fib_number::fib_number(element))
    }

}

mod extract_number;
mod fib_number;