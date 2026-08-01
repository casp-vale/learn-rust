use std::io;

fn main() {
    println!("Select your option: ");
    println!("1. Convert temperature from Fahrenheit to Celsius.");
    println!("2. Convert temperature from Celsius to Fahrenheit.");

    let selection: u32;

    // Loop until we get a valid selection (1 or 2)
    loop {
        let mut selection_str = String::new();

        io::stdin()
            .read_line(&mut selection_str)
            .expect("Failed to read line");

        match selection_str.trim().parse() {
            Ok(1) => {
                selection = 1;
                break;
            }
            Ok(2) => {
                selection = 2;
                break;
            }
            _ => {
                println!("Please provide a valid selection (1 or 2)!!");
                continue;
            }
        };
    }

    // Now process the valid choice
    if selection == 1 {
        // Loop until we get a valid float for temperature
        let temp_in_fahrenheit: f64 = loop {
            println!("Enter your temperature in Fahrenheit: ");
            let mut temp_str = String::new();

            io::stdin()
                .read_line(&mut temp_str)
                .expect("Failed to read line");

            match temp_str.trim().parse() {
                Ok(num) => break num, // break out of this sub-loop returning the number
                Err(_) => {
                    println!("That's not a valid number! Please try again.");
                }
            }
        };

        let result: f64 = fahrenheit_to_celsius(temp_in_fahrenheit);
        println!("The converted temperature from fahrenheit to celsius is: {result:.2}°C");

    } else if selection == 2 {
        // Loop until we get a valid float for temperature
        let temp_in_celsius: f64 = loop {
            println!("Enter your temperature in Celsius: ");
            let mut temp_str = String::new();

            io::stdin()
                .read_line(&mut temp_str)
                .expect("Failed to read line");

            match temp_str.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("That's not a valid number! Please try again.");
                }
            }
        };

        let result: f64 = celsius_to_fahrenheit(temp_in_celsius);
        println!("The converted temperature from celsius to fahrenheit is: {result:.2}°F");
    }
}

// Function: temperature from fahrenheit to celsius
fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

// Function: temperature from celsius to fahrenheit 
fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}