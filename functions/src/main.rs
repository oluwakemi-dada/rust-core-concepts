// ---- Intro to Functions | Parameters and Arguments ----
fn main() {
    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Queens");
    bake_pizza(15, "mushroom");
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(num: i32, topping: &str) {
    println!("Baking {num} {topping} pizza(s)");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}
