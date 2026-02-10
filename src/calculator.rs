use std::io;

pub fn run() {
    println!("\x1b[34m=================================\x1b[0m");
    println!("\x1b[34m   Simple Rust CLI Calculator\x1b[0m");
    println!("\x1b[34m=================================\x1b[0m");
    println!();

    loop {
        let num1: f64 = match get_number("Enter first number (or 'q' to quit): ") {
            Some(n) => n,
            None => {
                println!("\x1b[34mGoodbye!\x1b[0m");
                break;
            }
        };

        let operator = get_input("Enter operator (+, -, *, /, %, ^): ");
        let operator = operator.trim();

        let num2: f64 = match get_number("Enter second number: ") {
            Some(n) => n,
            None => {
                println!("Invalid number. Try again.");
                continue;
            }
        };

        match calculate(num1, num2, operator) {
            Some(result) => {
                println!();
                println!("\x1b[34m  {} {} {} = {:.4}\x1b[0m", num1, operator, num2, result);
                println!();
            }
            None => {
                println!("Error: Invalid operation.");
                println!();
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("\x1b[34m{}\x1b[0m", prompt);
    use std::io::Write;
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn get_number(prompt: &str) -> Option<f64> {
    let input = get_input(prompt);
    let trimmed = input.trim();

    if trimmed.eq_ignore_ascii_case("q") || trimmed.eq_ignore_ascii_case("quit") {
        return None;
    }

    match trimmed.parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("'{}' is not a valid number.", trimmed);
            None
        }
    }
}

fn calculate(a: f64, b: f64, operator: &str) -> Option<f64> {
    match operator {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => {
            if b == 0.0 {
                println!("Error: Division by zero!");
                None
            } else {
                Some(a / b)
            }
        }
        "%" => Some(a % b),
        "^" => Some(a.powf(b)),
        _ => {
            println!("Unknown operator: '{}'. Use +, -, *, /, %, or ^.", operator);
            None
        }
    }
}
