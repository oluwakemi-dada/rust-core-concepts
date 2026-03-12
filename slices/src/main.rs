// ---- Create a String Slice from a String ----
// fn main() {
//     let action_hero = String::from("Arnold Schwarzenegger");
//     let first_name = &action_hero[0..6];
//     println!("{first_name}");

//     let last_name = &action_hero[7..21];
//     println!("{last_name}")
// }

// ---- String Slices and String Literals ----
// fn main() {
//     let first_name = {
//         let action_hero = "Arnold Schwarzenegger";
//         &action_hero[0..6]
//     };

//     println!("{first_name}");
// }

// ---- String Slice Lengths ----
// fn main() {
//     let food = "🍕";
//     println!("{}", food.len());

//     let pizza_slice = &food[0..4];
//     println!("{}", pizza_slice.len())
// }

// ---- Syntactic Shortcuts ----
// fn main() {
//     let action_hero = String::from("Arnold Schwarzenegger");
//     let first_name = &action_hero[..6];
//     println!("His first name is {first_name}");

//     let last_name = &action_hero[7..];
//     println!("His last name is {last_name}");

//     let full_name = &action_hero[..];
//     println!("His full name is {full_name}");
// }

// ---- String Slices as Function Parameters ----
fn do_hero_stuff(hero_name: &str) { // &String -> &str
    println!("{hero_name} saves the day");
}

fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);

    let another_action_hero = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);
}
