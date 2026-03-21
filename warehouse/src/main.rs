// ---- Intro to Modules | The pub Keyword | The Benefit of Namespaces ----
mod inventory {
    const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Ivan Inventory";

    #[derive(Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive(Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }

    fn talk_to_manager() {
        println!("Hey, {MANAGER}, how's your coffee");
    }
}

mod orders {
    pub const MANAGER: &str = "Oliver Orderson";
}

fn main() {
    println!("The manager of our inventory is {}", inventory::MANAGER);
    println!("The manager of our orders is {}", orders::MANAGER);
}
