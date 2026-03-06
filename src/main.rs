mod grouping;
mod Assignments;
mod error_handling;
mod file_system;
mod food_registry;
use file_system::main as file_system;
use error_handling::error;
use food_registry::{Food, FoodRegistry, FoodType};

use grouping::group;
const X: u32 = 5;

fn shadowing() {
    let x = 5;
    let x = 5 + 4;
    let x = x * 3;
    println!("{}", x);
}

#[derive(Debug)]
enum Option{
    None,
    Some(String)
}


fn main() {
    run_food_registry();
    file_system();
    error();
    let sample = Option::None;
    match sample {
        Option::Some(x) =>{
            println!("1 a value exist and it {}", x);
        }
        Option::None => {
            println!("1 no value ");
        }
    }
    let sample = Option::Some("hello".to_string());
    match sample {
        Option::Some(x) =>{
            println!("2 a value exist and it {}", x);
        }
        Option::None => {
            println!("2 no value ");
        }
    }

    let sample = Some(9);
    match sample {
        Some(x) =>{
            println!("2 a value exist and it {}", x);
        }
        _ => {
            println!("2 no value ");
        }
    }

    let result:Result<&str, String> = Ok("hello");

    match result {
        Ok(x)=>{
            println!("result value {}",x);
        }
        Err(error)=>{
            println!("error {}",error);
        }

    }
    group();
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
    println!(
        "My user name is {}, \n age is {}, \n email is {}, \n is_active is {}",
        name, age, email, is_active
    );
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

fn run_food_registry() {
    let mut registry = FoodRegistry::new();

    // Create food items
    let snack = Food {
        name: String::from("Chin Chin"),
        price: 500,
        calories: 200,
    };

    let swallow = Food {
        name: String::from("Pounded Yam"),
        price: 1500,
        calories: 600,
    };

    let rice = Food {
        name: String::from("Jollof Rice"),
        price: 1200,
        calories: 550,
    };

    let beans = Food {
        name: String::from("Beans Porridge"),
        price: 800,
        calories: 450,
    };

    // Add foods
    registry.add_food(FoodType::Snack, snack).unwrap();
    registry.add_food(FoodType::Swallow, swallow).unwrap();
    registry.add_food(FoodType::Rice, rice).unwrap();
    registry.add_food(FoodType::Beans, beans).unwrap();

    // Fetch a food
    match registry.get_food(&FoodType::Rice) {
        Ok(food) => println!("Found food: {:?}", food),
        Err(e) => println!("Error: {:?}", e),
    }
}
