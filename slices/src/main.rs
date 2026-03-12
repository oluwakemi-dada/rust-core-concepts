// ---- Create a String Slice from a String ----
// fn main() {
//     let action_hero = String::from("Arnold Schwarzenegger");
//     let first_name = &action_hero[0..6];
//     println!("{first_name}");

//     let last_name = &action_hero[7..21];
//     println!("{last_name}")
// }

// ---- String Slices and String Literals ----
fn main() {
    let first_name = {
        let action_hero = "Arnold Schwarzenegger";
        &action_hero[0..6]
    };

    println!("{first_name}");
}
