// ----- Review of Strings -----
// fn main() {
//     let pirate = "Bloodhook";

//     let empty = String::new();

//     let sailor = String::from(pirate);

//     let bad_guy = pirate.to_string();

//     println!("{pirate} and {sailor} and {bad_guy}");

//     let first_initial = &pirate[0..1];
//     println!("{first_initial}")
// }

// ----- Concatenation -----
fn main() {
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let full_name = first_name + &last_name;
    // let full_name = first_name.clone() + &last_name;
    println!("{full_name}");
    println!("{first_name}")
}
