fn main() {
    let mut food_registry: Vec<Fooddata> = Vec::new();
    let pounded_yam = Fooddata {
        name: String::from("pounded yam"),
        expired: false,
        category: Foodchoice::swallow,
        price: 200.00,
    };

    let jollof = Fooddata {
        name: String::from("jollof  rice"),
        expired: false,
        category: Foodchoice::rice,
        price: 100.00,
    };

    let burger = Fooddata {
        name: String::from("sweet burger"),
        expired: false,
        category: Foodchoice::snack,
        price: 50.00,
    };

    let porridge = Fooddata {
        name: String::from("porridge beans"),
        expired: true,
        category: Foodchoice::beans,
        price: 150.00,
    };

    food_registry.push(burger);
    food_registry.push(porridge);
    food_registry.push(jollof);
    food_registry.push(pounded_yam);

    for item in &food_registry {
        validate_food(item)
            .map_err(|e| {
                eprintln!("Error: {}", e);
            })
            .map(|valid_item| {
                if valid_item.expired {
                    eprintln!("Warning: '{}' is expired!", valid_item.name);
                }
                println!("{:#?}", valid_item);
            })
            .ok();
    }
    
}

fn validate_food(item: &Fooddata) -> Result<&Fooddata, String> {
    if item.price < 0.0 {
        return Err(format!("'{}' has a negative price!", item.name));
    }
    if item.name.trim().is_empty() {
        return Err("Food item has an empty name!".to_string());
    }
    Ok(item)
}

#[derive(Debug)]
enum Foodchoice {
    snack,
    swallow,
    rice,
    beans,
}

#[derive(Debug)]
struct Fooddata {
    name: String,
    expired: bool,
    category: Foodchoice,
    price: f64,
}
