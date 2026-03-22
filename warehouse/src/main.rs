use fake::{Fake, Faker};

use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, Item, ORDERS_MANAGER, ProductCategory};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    let fake_item: Item = Faker.fake();
    println!("{:#?}", fake_item);

    let random_category: ProductCategory = Faker.fake();
    println!("{:#?}", random_category)
}
