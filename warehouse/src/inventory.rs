pub mod products;

pub const FLOOR_SPACE: i32 = 10_000;
pub const MANAGER: &str = "Ivan Inventory";

fn talk_to_manager() {
    println!("Hey, {}, how's your coffee", MANAGER);
}
