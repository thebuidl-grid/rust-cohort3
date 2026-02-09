use std::io;

const X: u32 = 5;

fn shadowing() {
    let x = 5;
    let x = 5 + 4;
    let x = x * 3;
    println!("{}", x);
}

fn main() {
    let mut name = "John";
    println!("Hello, world! {}", X);
    println!("{}", name);

    name = "Jane";
    println!("{}", name);
    println!(" ------ {}", X);

    let name = String::from("martin");

    school_conditionals();
    loops();
    while_loop();

    // 🔽 CLI Calculator
    calculator_cli();
}

fn user_name(name: String) {
    println!("My user name is {}", name)
}

fn add(a: u32, b: u32) -> u32 {
    let sum = a + b;
    println!("The sum of {a} and {b} is {sum}");
    sum
}

fn sub(a: u32, b: u32) -> u32 {
    let sum = a - b;
    add(a, b);
    println!("The sum of {a} and {b} is {sum}");
    sum
}

fn user(name: &str, age: u32, email: String, is_active: bool) -> String {
    println!(
        "My user name is {}, \n age is {}, \n email is {}, \n is_active is {}",
        name, age, email, is_active
    );
    name.to_string()
}

fn conditionals() {
    let age = 20;

    if age > 18 {
        println!("You are an adult");
    } else if age == 18 {
        println!("You just became an adult")
    } else {
        println!("you are a minor")
    }
}

fn school_conditionals() {
    let time: u32 = 19;

    if time < 8 {
        println!("You're early!")
    } else if time > 8 && time < 10 {
        println!("You're late and should be punished!")
    } else if time == 10 {
        println!("It's break time!")
    } else if time == 11 {
        println!("Break Over, Go back to class!")
    } else if time > 11 && time < 15 {
        println!("You should be in class!")
    } else if time == 15 {
        println!("It's Closing time!")
    } else {
        println!("You can do whatever you want after closing!")
    }
}

fn loops() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
        println!("Infinite loop {}", count);
    };

    println!("The result is {:?}", result);
}

fn while_loop() {
    let mut count = 6;

    while count != 0 {
        println!("The count is {}", count);
        count -= 1;
    }
}

fn calculator_cli() {
    let mut input = String::new();

    println!("\n--- CLI Calculator ---");

    println!("Enter first number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Invalid number");

    input.clear();

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operator = input.trim().to_string();

    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Invalid number");

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}
