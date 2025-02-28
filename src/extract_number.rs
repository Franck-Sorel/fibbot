pub fn collect(text: &str ) -> Vec<u128> {

        // let text = "Here are some numbers: 42, 12345678901234567890, and -7, but not 3.14 or 0.";
        let mut numbers: Vec<u128> = Vec::new();
        let mut current_number = String::new();
    
        for c in text.chars() {
            if c.is_digit(10) { // Check if the character is a digit
                current_number.push(c); // Collect the digit
            } else {
                if !current_number.is_empty() { // If we have collected digits
                    // Convert the collected digits to u128 and store it
                    if let Ok(num) = current_number.parse::<u128>() {
                        numbers.push(num);
                    } else {
                        eprintln!("Error: Number is too large to fit in u128.");
                    }
                    current_number.clear(); // Clear the current number for the next sequence
                }
            }
        }
    
        // Check if there's a number left to add after the loop
        if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<u128>() {
                numbers.push(num);
            } else {
                eprintln!("Error: Number is too large to fit in u128.");
            }
        }
    
        numbers

}
