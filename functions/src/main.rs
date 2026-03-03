// ---- Intro to Functions | Parameters and Arguments ----
// fn main() {
//     open_store("Brooklyn");
//     bake_pizza(20, "pepperoni");
//     swim_in_profit();
//     swim_in_profit();
//     swim_in_profit();
//     open_store("Queens");
//     bake_pizza(15, "mushroom");
// }

// fn open_store(neighborhood: &str) {
//     println!("Opening my pizza store in {neighborhood}");
// }

// fn bake_pizza(num: i32, topping: &str) {
//     println!("Baking {num} {topping} pizza(s)");
// }

// fn swim_in_profit() {
//     println!("So much $$$, so little time");
// }

// ---- Explicit Return Values | Implicit Return Values ----
// fn main() {
//     let result = square(5);
//     println!("The square of 5 is {result}");

//     let result = square(13);
//     println!("The square of 13 is {result}");
// }

// fn square(number: i32) -> i32 {
//     // return number * number;
//     number * number
// }

// ---- The Unit as a Return Type ----
// fn main() {
//     let result = mystery();
// }

// fn mystery() {
//     println!("Hello there");
// }

// ---- Block in Functions ----
// fn main() {
//     let multiplier = 3;

//     let calculation = {
//         let value = 5 + 4;
//         value * multiplier
//     };

//     println!("{calculation}")
// }

// ---- Coding Challenge ----
fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains('a'), text.contains('b'))
}

fn main() {
    apply_to_jobs(35, "Rust Developer");
    println!("{}", is_even(6));
    println!("{}", is_even(3));

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
}
