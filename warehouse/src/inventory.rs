pub mod products;

pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey, {}, how's your coffee", MANAGER);
    println!("{:?}", crate::inventory::products::ProductCategory::Ladder);
    println!("{:?}", products::ProductCategory::Hammer);
}
