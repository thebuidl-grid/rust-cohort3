use std::io;

pub fn run() {
    println! ("Enter calculation (5 + 3):");

    let mut input = String::new();
    io::stdin()
         .read_line(&mut input)
         .expect("Fail to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("Invalid fomat. Use: number operator number");
        return;
    }

    let num1: f64 = parts[0].parse().expect("Invalid number");
    let operator = parts[1];
    let num2: f64 = parts[2].parse().expect("Invalid number");

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0{
                println!("Cannot divide by zero");
                return;
            }
            num1 / num2
        }
        _=> {
            println!("Invalide Operator");
            return;
          }  

    };

    println! ("Result: {}", result);

}