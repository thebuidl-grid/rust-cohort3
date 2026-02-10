mod calculator;

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

    // let y = x + 23;
    // X = 23;
    name = "Jane";
    println!("{}", name);
    println!(" ------ {}", X);

    // shadowing();
    let name = String::from("martin");
    // user_name(name);
    // user_name("Chris".to_string());
    // user_name("Emma".to_string());

    // sub(20, 10);

    // user("Mark", 23, "mark@gmail.co".to_string(), true);

    // conditionals();

    school_conditionals();
    loops();
    while_loop();

    xoulomon_calculator::run();
}

fn user_name(name: String) {
    println!("My user name is {}", name)
}

fn add(a: u32, b: u32) -> u32 {
    let sum = a + b;

    println!("The sum of {a} and {b} is {sum}");
    return sum;
}

fn sub(a: u32, b: u32) -> u32 {
    let sum = a - b;
    add(a, b);

    println!("The sum of {a} and {b} is {sum}");
    return sum;
}

fn user(name: &str, age: u32, email: String, is_active: bool) -> String {
    println!("My user name is {}, \n age is {}, \n email is {}, \n is_active is {}", name, age, email, is_active);
    return name.to_string();
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
        println!("You can do what ever you want after closing!")
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
