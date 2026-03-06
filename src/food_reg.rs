use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum categories1 {
    rice,
    beans,
    drinks,
    snacks,
    swallow,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FoodType {
    pub name: String,
    pub price: u32,
    pub is_vegan: bool,
    pub category: categories1,
    pub expiration: SystemTime,
}

pub fn expiration(Food_list: Vec<FoodType>) -> Result<(), String> {
    let current_time = SystemTime::now();

    for food in Food_list {
        if current_time > food.expiration {
            return Err(String::from("Food has expired"));
        }
    }
    Ok(())
}
