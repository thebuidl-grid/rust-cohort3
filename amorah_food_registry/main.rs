use std::collections::HashMap;
use std::fmt;

// ----------------------------
// Food Type
// ----------------------------
#[derive(Debug, Clone)]
enum FoodType {
    Fruit,
    Vegetable,
    Grain,
    Protein,
    Dairy,
    Other,
}

// ----------------------------
// Food Details Struct
// ----------------------------
#[derive(Debug, Clone)]
struct Food {
    name: String,
    calories: u32,
    food_type: FoodType,
}

// ----------------------------
// Custom Error Type
// ----------------------------
#[derive(Debug)]
enum RegistryError {
    FoodAlreadyExists,
    FoodNotFound,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::FoodAlreadyExists => write!(f, "Food already exists in registry"),
            RegistryError::FoodNotFound => write!(f, "Food not found in registry"),
        }
    }
}

// ----------------------------
// Food Registry
// ----------------------------
struct FoodRegistry {
    foods: HashMap<String, Food>,
}

impl FoodRegistry {
    fn new() -> Self {
        Self {
            foods: HashMap::new(),
        }
    }

    // Add Food
    fn add_food(&mut self, food: Food) -> Result<(), RegistryError> {
        if self.foods.contains_key(&food.name) {
            return Err(RegistryError::FoodAlreadyExists);
        }

        self.foods.insert(food.name.clone(), food);
        Ok(())
    }

    // Get Food
    fn get_food(&self, name: &str) -> Result<&Food, RegistryError> {
        self.foods.get(name).ok_or(RegistryError::FoodNotFound)
    }

    // Remove Food
    fn remove_food(&mut self, name: &str) -> Result<(), RegistryError> {
        self.foods
            .remove(name)
            .map(|_| ())
            .ok_or(RegistryError::FoodNotFound)
    }

    // List Foods
    fn list_foods(&self) {
        for food in self.foods.values() {
            println!("{:?}", food);
        }
    }
}

// ----------------------------
// Example Usage
// ----------------------------
fn main() {
    let mut registry = FoodRegistry::new();

    let apple = Food {
        name: "Apple".to_string(),
        calories: 95,
        food_type: FoodType::Fruit,
    };

    let rice = Food {
        name: "Rice".to_string(),
        calories: 200,
        food_type: FoodType::Grain,
    };

    registry.add_food(apple).unwrap();
    registry.add_food(rice).unwrap();

    match registry.get_food("Apple") {
        Ok(food) => println!("Found food: {:?}", food),
        Err(e) => println!("Error: {}", e),
    }

 
    registry.list_foods();
}