// ---- Scope and Ownership ----
// fn main() {
//     let age = 33;
//     let is_handsome = true;

//     println!("{age}");
//     println!("{is_handsome}");

//     // age variable exists here
// } // is_handsome goes out of scope, then age variable goes out of scope here

// ---- The Copy Trait ----
fn main() {
    let time = 2026;
    let years = time;

    println!("The time is {time}. It is the year {years}.");
}
