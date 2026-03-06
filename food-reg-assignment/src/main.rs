use std::collections::HashMap;

#[derive(Debug, Clone)]
enum FoodType {
    Snacks,
    Swallow,
    Grains,
    Pastas,
}

#[derive(Debug)]
enum RegistryError {
    ExpiredFood,
    NotFound,
    InvalidInput,
}

#[derive(Debug, Clone)]
struct Food {
    name: String,
    ingredients: Vec<String>,
    category: FoodType,
    is_expired: bool,
}

#[derive(Debug)]
struct Registry {
    foods: HashMap<String, Food>,
}

impl Registry {
    fn new() -> Self {
        Self {
            foods: HashMap::new(),
        }
    }

    fn add_food(&mut self, food: Food) -> Result<(), RegistryError> {
        if food.is_expired {
            return Err(RegistryError::ExpiredFood);
        }
        let key = food.name.clone();
        self.foods.insert(key, food);
        Ok(())
    }

    fn get_food(&self, key: &str) -> Result<&Food, RegistryError> {
        self.foods.get(key).ok_or(RegistryError::NotFound)
    }

    fn eat_food(&self, key: &str) -> Result<String, RegistryError> {
        let food = self.get_food(key)?;

        if food.is_expired {
            return Err(RegistryError::ExpiredFood);
        }

        Ok(format!("You can eat {}", food.name))
    }
}

fn parse_expiry_flag(input: &str) -> Result<bool, RegistryError> {
    input.parse::<bool>().map_err(|_| RegistryError::InvalidInput)
}

fn main() {
    let mut registry = Registry::new();

    let rice = Food {
        name: "Jollof Rice".to_string(),
        ingredients: vec!["Rice".to_string(), "Tomato".to_string(), "Pepper".to_string(), "Kpomo".to_string(), "fish".to_string(),"meat".to_string()],
        category: FoodType::Grains,
        is_expired: false,
    };

    let pie_expired_flag = parse_expiry_flag("true").unwrap_or(true);
    let meat_pie = Food {
        name: "Meat Pie".to_string(),
        ingredients: vec!["Flour".to_string(), "Meat".to_string(), "Butter".to_string(), "potatoes".to_string(),"nut meg".to_string()],
        category: FoodType::Snacks,
        is_expired: pie_expired_flag,
    };

    match registry.add_food(rice) {
        Ok(()) => println!("Added Jollof Rice"),
        Err(err) => println!("Could not add Jollof Rice: {:?}", err),
    }

    match registry.add_food(meat_pie) {
        Ok(()) => println!("Added Meat Pie"),
        Err(err) => println!("Could not add Meat Pie: {:?}", err),
    }

    match registry.get_food("Jollof Rice") {
        Ok(food) => println!("Found food: {:?}", food),
        Err(err) => println!("Get food error: {:?}", err),
    }

    match registry.eat_food("Jollof Rice") {
        Ok(message) => println!("{}", message),
        Err(err) => println!("Eat food error: {:?}", err),
    }

    match registry.eat_food("Meat Pie") {
        Ok(message) => println!("{}", message),
        Err(err) => println!("Eat food error: {:?}", err),
    }
}
