use std::io;

pub fn cal() {
    println!("this is how values are to be inputed 3 + 2");

    let mut  input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    let a: f64 = parts[0].parse().unwrap();
    let op = parts[1];
    let b: f64 = parts[2].parse().unwrap();

    let answer = match op{
        "+" => a+b,
        "-" => a-b,
        "*" => a*b,
        "/" => a/b,
        _ => 0.0,
    };
    println!("answer is this {}", answer);
}


