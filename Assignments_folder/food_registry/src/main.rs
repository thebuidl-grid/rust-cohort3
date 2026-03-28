use std::collections::HashMap;
use std::fmt;
use thiserror::Error;
use chrono::{DateTime, Utc, Duration};


#[derive(Error, Debug)]
pub enum FoodError {
    #[error("Food '{0}' is expired! Cannot eat 😭")]
    Expired(String),

    #[error("Food '{food}' not found in registry")]
    NotFound { food: String },

    #[error("Invalid operation: {0}")]
    Invalid(String),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner, 
}

impl fmt::Display for MealType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MealType::Breakfast => write!(f, "Breakfast"),
            MealType::Lunch     => write!(f, "Lunch"),
            MealType::Dinner   => write!(f, "Dinner"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FoodCategory {
    Snack,
    Swallow,
    Pasta,
    Rice,
}


#[derive(Debug, Clone)]
pub struct Food {
    pub name: String,
    pub category: FoodCategory,
    pub meal_type: MealType,            
    pub produced_at: DateTime<Utc>,
    /// How many hours this food stays good after production
    pub shelf_life_hours: u32,
}

impl Food {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let expiry = self.produced_at + Duration::hours(self.shelf_life_hours as i64);
        now > expiry
    }

    pub fn eat(&self) -> Result<&str, FoodError> {
        if self.is_expired() {
            return Err(FoodError::Expired(self.name.clone()));
        }
        Ok("This food sweet die!")
    }
}


#[derive(Debug, Default)]
pub struct FoodRegistry {
    foods: HashMap<String, Food>,
}

impl FoodRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, food: Food) {
        self.foods.insert(food.name.to_lowercase(), food);
    }

    pub fn get(&self, name: &str) -> Result<&Food, FoodError> {
        let key = name.to_lowercase();
        self.foods
            .get(&key)
            .ok_or(FoodError::NotFound { food: name.to_string() })
    }

    pub fn try_eat(&self, name: &str) -> Result<String, FoodError> {
        let food = self.get(name)?;
        let message = food
            .eat()
            .map_err(|e| FoodError::Invalid(format!("Cannot eat {}: {}", name, e)))?;
        Ok(format!("{} → {}", name, message))
    }

    pub fn list_by_meal_type(&self, meal: MealType) -> Vec<&Food> {
        self.foods
            .values()
            .filter(|f| f.meal_type == meal)
            .collect()
    }

    pub fn list_all(&self) -> Vec<&Food> {
        self.foods.values().collect()
    }
}


fn main() {
    let mut registry = FoodRegistry::new();

    // Akara (very common Nigerian breakfast)
    let akara = Food {
        name: "Akara".to_string(),
        category: FoodCategory::Snack,
        meal_type: MealType::Breakfast,
        produced_at: Utc::now() - Duration::hours(4),
        shelf_life_hours: 12,
    };

    // Meat Pie (can be breakfast or snack)
    let meat_pie = Food {
        name: "Meat Pie".to_string(),
        category: FoodCategory::Snack,
        meal_type: MealType::Lunch,  
        produced_at: Utc::now() - Duration::hours(18),
        shelf_life_hours: 24,
    };

    // Jollof Rice (classic lunch / dinner)
    let jollof = Food {
        name: "Jollof Rice".to_string(),
        category: FoodCategory::Rice,
        meal_type: MealType::Lunch,
        produced_at: Utc::now() - Duration::hours(6),
        shelf_life_hours: 48,
    };

    // Fried Rice (classic lunch / dinner)
    let fried = Food {
        name: "Fried Rice".to_string(),
        category: FoodCategory::Rice,
        meal_type: MealType::Lunch,
        produced_at: Utc::now() - Duration::hours(6),
        shelf_life_hours: 48,
    };

    // Pounded Yam + Egusi (typical evening / dinner swallow)
    let pounded_yam_egusi = Food {
        name: "Pounded Yam & Egusi Soup".to_string(),
        category: FoodCategory::Swallow,
        meal_type: MealType::Dinner,
        produced_at: Utc::now() - Duration::hours(10),
        shelf_life_hours: 36,
    };

    // Tuwon Chinkafa + Miyan Geda (typical evening / dinner swallow)
    let tuwon_chinkafa_miyan_geda = Food {
        name: "Tuwon Chinkafa & Miyan Geda ".to_string(),
        category: FoodCategory::Swallow,
        meal_type: MealType::Dinner,
        produced_at: Utc::now() - Duration::hours(10),
        shelf_life_hours: 36,
    };

    registry.register(akara);
    registry.register(meat_pie);
    registry.register(jollof);
    registry.register(fried);
    registry.register(pounded_yam_egusi);
    registry.register(tuwon_chinkafa_miyan_geda);

    // ─── Try eating ───────────────────────────────────────
    println!("Trying to eat some foods:");
    for name in ["Akara", "Jollof Rice", "Meat Pie"] {
        match registry.try_eat(name) {
            Ok(msg)  => println!("  ✓ {}", msg),
            Err(e)   => println!("  ✗ {}", e),
        }
    }

    // ─── Show by meal type ────────────────────────────────
    println!("\nBreakfast options:");
    for food in registry.list_by_meal_type(MealType::Breakfast) {
        println!("  • {} (expires in ~{}h)", 
            food.name,
            (food.produced_at + Duration::hours(food.shelf_life_hours as i64) - Utc::now()).num_hours()
        );
    }

    println!("\nEvening options:");
    for food in registry.list_by_meal_type(MealType::Dinner) {
        println!("  • {}", food.name);
    }
}