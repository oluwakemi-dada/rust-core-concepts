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
fn main() {
    let season = "summer";

    if season == "summer" {
        println!("School's out!");
    } else if season == "winter" {
        println!("Brr, so cold!");
    } else if season == "fall" {
        println!("Leaves falling!")
    } else if season == "spring" {
        println!("Lots of rain!")
    }
}
