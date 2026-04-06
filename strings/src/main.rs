// ----- Review of Strings -----
fn main() {
    let pirate = "Bloodhook";

    let empty = String::new();

    let sailor = String::from(pirate);

    let bad_guy = pirate.to_string();

    println!("{pirate} and {sailor} and {bad_guy}");

    let first_initial = &pirate[0..1];
    println!("{first_initial}")
}
