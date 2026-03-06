use std::collections::HashMap;

// ==========================
// Struct: Food Details
// ==========================
#[derive(Debug, Clone)]
pub struct Food {
    pub name: String,
    pub price: u32,
    pub calories: u32,
}

// ==========================
// Enums: Different Food Types
// ==========================
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum FoodType {
    Snack,
    Swallow,
    Rice,
    Beans,
}

// ==========================
// Enum: Custom Errors
// ==========================
#[derive(Debug)]
pub enum RegistryError {
    FoodAlreadyExists,
    FoodNotFound,
}

// ==========================
// Food Registry Struct
// ==========================
pub struct FoodRegistry {
    foods: HashMap<FoodType, Food>,
}

impl FoodRegistry {
    pub fn new() -> Self {
        Self {
            foods: HashMap::new(),
        }
    }

    // Add food to registry
    pub fn add_food(&mut self, food_type: FoodType, food: Food) -> Result<(), RegistryError> {
        if self.foods.contains_key(&food_type) {
            return Err(RegistryError::FoodAlreadyExists);
        }
        self.foods.insert(food_type, food);
        Ok(())
    }

    // Get food from registry
    pub fn get_food(&self, food_type: &FoodType) -> Result<&Food, RegistryError> {
        self.foods.get(food_type).ok_or(RegistryError::FoodNotFound)
    }
}
