// ---- Define a Struct | Struct instance ----
fn main() {
    struct Coffee {
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
}
