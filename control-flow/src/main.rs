// ---- The if Statement ----
// fn main() {
//     let some_condition_that_we_cannot_predict_in_advance = true;

//     if some_condition_that_we_cannot_predict_in_advance {
//         println!("This line will be output");
//     }

//     if false {
//         println!("This line will NOT be output");
//     }
// }

// ---- The else if Statement ----
// fn main() {
//     let season = "summer";

//     if season == "summer" {
//         println!("School's out!");
//     } else if season == "winter" {
//         println!("Brr, so cold!");
//     } else if season == "fall" {
//         println!("Leaves falling!")
//     } else if season == "spring" {
//         println!("Lots of rain!")
//     }
// }

// ---- The else Statement ----
// fn main() {
//     let season = "spring";

//     if season == "summer" {
//         println!("School's out!");
//     } else if season == "winter" {
//         println!("Brr, so cold!");
//     } else {
//         println!("Lots of rain!");
//     }
// }

// ---- Assigning Result of if else Statement to Variable ----
// fn even_or_odd(number: i32) {
//     let result = if number % 2 == 0 { "even" } else { "odd" };
//     println!("The number is {result}")
// }
// fn main() {
//     even_or_odd(17);
//     even_or_odd(100);
// }

// ---- The Match Statement ----
fn main() {
    let evaluation = true;

    let value = match evaluation {
        true => 20,
        false => 40,
    };

    println!("{value}")
}
