// ----- The panic! Macro -----
// fn main() {
//     let num = 2;
//     panic!("Something went wrong ");
// }

// ----- The process Module and the exit Function -----
// use std::process;
// fn main() {
//     // process::exit(0);

//     process::exit(1);
//     println!("This will not print");
// }

// ----- Standard Error (eprintln! Macro) -----
fn main() {
    println!("Some status update");
    eprintln!("Error: Some error message");
}
