mod inventory;
mod orders;

use inventory::FLOOR_SPACE;
use inventory::products::{Item, ProductCategory};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );

    let favorite_category = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_category:?}");

    let tall_ladder = Item::new(String::from("Ladder-o-matic 2000"), favorite_category, 100);
    println!("{:#?}", tall_ladder)
}
