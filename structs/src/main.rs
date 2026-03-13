// ---- Define a Struct | Struct instance | Access Struct Fields ----
fn main() {
    struct Coffee {
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,
    };

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favorite_coffee = mocha.name;
    println!("{favorite_coffee}");
    // println!("{}", mocha.name)
}
