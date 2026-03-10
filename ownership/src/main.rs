// ---- Scope and Ownership ----
// fn main() {
//     let age = 33;
//     let is_handsome = true;

//     println!("{age}");
//     println!("{is_handsome}");

//     // age variable exists here
// } // is_handsome goes out of scope, then age variable goes out of scope here

// ---- The Copy Trait ----
// fn main() {
//     let time = 2026;
//     let years = time;

//     println!("The time is {time}. It is the year {years}.");
// }

// ---- The String Type ----
// fn main() {
//     let text = String::new();
//     let candy = String::from("KitKat");
// }

// ---- The push_str Method on a String Type ----
// fn main() {
//     let mut name = String::from("Oluwakemi");
//     println!("{name}");

//     name.push_str(" Olayemi");
//     println!("{name}");

//     name.push_str(" Dada");
//     println!("{name}");
// }

// ---- Moves and Ownership ----
// fn main() {
//     let person = String::from("Oluwakemi");
//     let genius = person;

//     // println!("My name is {person}")
// }

// ---- The drop Function ----
// fn main() {
//     let person = String::from("Oluwakemi");

//     drop(person);

//     println!("{person}");

//     let genius = person;

//     println!("Genius {genius}");
// }

// ---- The clone Method ----
// fn main() {
//     let person = String::from("Oluwakemi");
//     let genius = person.clone();

//     println!("My name is {person}")
// }

// ---- References and Borrowing ----
// fn main() {
//     let my_stack_value = 2;
//     let my_integer_reference = &my_stack_value;

//     let my_heap_value = String::from("Toyota");
//     let my_heap_reference = &my_heap_value;
// }

// ---- Dereference Operator ----
// fn main() {
//     let my_stack_value = 2;
//     let my_integer_reference = &my_stack_value;

//     println!("{}", my_integer_reference);
//     println!("{}", *my_integer_reference);

//     let my_heap_value = String::from("Toyota");
//     let my_heap_reference = &my_heap_value;

//     println!("{}", my_heap_reference);
//     println!("{}", *my_heap_reference);
// }

// ---- Ownership and Function Parameters ----
// fn main() {
//     let oranges = String::from("Oranges");
//     print_my_value(oranges);
//     println!("{oranges} is still valid");
// }

// fn print_my_value(value: String) {
//     println!("Your value is {value}");
// }

// ---- Mutable Parameters ----
fn main() {
    let burger = String::from("Burger");
    add_fries(burger);
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}
