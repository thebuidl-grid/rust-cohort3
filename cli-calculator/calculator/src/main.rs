use std::io;

fn main() {
    println!("Welcome to my Rust cli calculator");
    println!("(type 'q' to quit)");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();

        println!("\nEnter your first number:");
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number, try again");
                continue;
            }
        };

        println!("Enter operator (+, -, *, /) or 'q' to quit:");
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        if operator == "q" {
            println!("Goodbye!");
            break;
        }

        println!("\nEnter your second number:");
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number, try again");
                continue;
            }
        };

        if operator == "+" {
            println!("Result: {}", num1 + num2);
        } else if operator == "-" {
            println!("Result: {}", num1 - num2);
        } else if operator == "*" {
            println!("Result: {}", num1 * num2);
        } else if operator == "/" {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Error: Cannot divide by zero.");
            }
        } else {
            println!("Unknown operator: {}", operator);
        }
    }
}
