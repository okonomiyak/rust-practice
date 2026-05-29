use std::vec;

use crate::Category::{Vegetable, Drink, Meat};

enum Category {
    Vegetable,
    Drink,
    Meat
}

struct Item {
    name: String,
    number: i32,
    category: Category
}
fn main() {
    let mut inventory:Vec<Item> = Vec::new();
    inventory.push(Item { name: String::from("apple"), number: 10000, category: Category::Vegetable });
    inventory.push(Item { name: String::from("coke"), number: 5000, category: Category::Drink });
    inventory.push(Item { name: String::from("poke"), number: 500, category: Category::Meat });
    println!("{:?}",inventory)
}