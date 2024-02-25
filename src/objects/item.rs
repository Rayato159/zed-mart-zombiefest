use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub name: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

fn spawn_item() {
    let item = Item::new("Potion".to_string());
    println!("Item: {:?}", item);
}