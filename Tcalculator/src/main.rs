use std::io;

fn main() {
    println!("Welcome to Tosin Cli Calculator!");

    // Initialize the num variables
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter the first number:");
    // Read the first numer input from the user
    io::stdin().read_line(&mut num1).expect("Failed to read");

    println!("Enter the second number:");
    // Read the second number input from the user
    io::stdin().read_line(&mut num2).expect("Failed to read");

    // Parse and convert the String user input to float
    let new_num1: f64 = num1.trim().parse().expect("Parsing failed");
    let new_num2: f64 = num2.trim().parse().expect("Parsing failed");
    let mut operator = String::new();

    println!("Enter an operator(+, -, /, *):");
    // Read the operator of choice from the user
    io::stdin().read_line(&mut operator).expect("Failed to read");

    // Run a match test to ascertain the operator choice of the user
    let result: f64 = match operator.trim() {
        "+" => new_num1 + new_num2,
        "-" => new_num1 - new_num2,
        "*" => new_num1 * new_num2,
        "/" => new_num1 / new_num2,
        _ => panic!("Invalid operator"),
    };
    
    println!("{}", result);
}

