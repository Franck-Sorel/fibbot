pub fn fib_number(input: u128) -> u128 {
    

    
    let mut prev: u128 = 0;
    let mut next: u128 = 1;

    if input == 0 {
        
        return 0;
    }

    let mut number: u128 = 1;
    for _ in 1..input {
        number = prev + next;
        prev = next;
        next = number;
    }
    
    number
}