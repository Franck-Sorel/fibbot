use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();



    // Check if the amount of arguments is exactly 3 (program name + 2 arguments)
    // if args.len() != 3 {
    //     eprintln!("Usage: {} <enable> <limit>", args[0]);
    //     std::process::exit(1);
    // }

    // bot trigger
    let _boool = args[1].parse::<bool>().expect("Err: Bad bool input");
    let boool = true;
    if !boool {
        return;
    }
    
    // threshold limit argument
    let _input: u128 = args[2]
        .trim()
        .parse()
        .expect("Enter an integer as argument two");

    let input = 100;

    // Pull request content as String
    let text = read_pull_request::read_pull();

    let text = match text {
        Ok(string) => string,
        Err(_) => std::process::exit(1),
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