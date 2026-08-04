use std::io;

fn main() {

    // The Fibonacci series is a sequence of numbers where 
    // each number is the sum of the two numbers immediately preceding it.

    let mut first_number: u128 = 0;
    let mut second_number: u128 = 1;

    // Loop until we get a valid input
    let input_number: u32 = loop {
        println!("Provide the nth number you want to print the fibonacci series to: ");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");

        match input_str.trim().parse() {
            Ok(num) => break num, // break out of this sub-loop returning the number
            Err(_) => {
                println!("That's not a valid number! Please try again.");
            }
        }
    };

    println!("\nPrinting the first {input_number} Fibonacci numbers:");

    for _ in 0..input_number {

        // Print the current term
        print!("{first_number} ");

        // Compute the next term using a temp variable
        let temp = first_number + second_number;

        // Shift the values forward
        first_number = second_number;
        second_number = temp;
    }
    
    println!(); // Prints a fresh newline at the end

}
