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
// fn main() {
//     let evaluation = true;

//     let value = match evaluation {
//         true => 20,
//         false => 40,
//     };

//     println!("{value}")
// }

// ---- Underscore in a Match Arm ----
// fn main() {
//     let season = "spring";

//     match season {
//         "summer" => println!("School's out!"),
//         "winter" => println!("Brr, so cold!"),
//         _ => println!("Lots of rain!"),
//     }
// }

// ---- The match Statement with Multiple Values and Conditionals ----
// fn main() {
//     let number = 3;

//     match number {
//         value if value % 2 == 0 => println!("{value} is an even number"),
//         value if value % 2 != 0 => println!("{value} is an odd number"),
//         _ => unreachable!(),
//     }
// }

// ---- The loop and break Keywords ----
// fn main() {
//     let mut seconds = 10;

//     loop {
//         if seconds == 0 {
//             println!("Blastoff!");
//             break;
//         }

//         println!("{seconds} seconds to blastoff...");
//         seconds -= 1;
//     }
// }

// ---- The continue Keyword ----
// fn main() {
//     let mut seconds = 21;

//     loop {
//         if seconds <= 0 {
//             println!("Blastoff!");
//             break;
//         }

//         if seconds % 2 == 0 {
//             println!("{seconds} seconds (even number), skipping 3 seconds...");
//             seconds -= 3;
//             continue;
//         }

//         println!("{seconds} seconds to blastoff...");
//         seconds -= 1;
//     }
// }

// ---- While loop ----
// fn main() {
//     let mut seconds = 21;

//     while seconds > 0 {
//         if seconds % 2 == 0 {
//             println!("{seconds} seconds (even number), skipping 3 seconds...");
//             seconds -= 3;
//             continue;
//         }

//         println!("{seconds} seconds to blastoff...");
//         seconds -= 1;
//     }

//     println!("Blastoff!");
// }

// ---- Recursion ----
// fn countdown(seconds: i32) {
//     if seconds == 0 {
//         println!("Blastoff!");
//     } else {
//         println!("{seconds} seconds to blastoff...");
//         countdown(seconds - 1);
//     }
// }
// fn main() {
//     countdown(5);
// }

// ---- Debugging ----
// fn countdown(seconds: i32) {
//     if seconds == 0 {
//         println!("Blastoff!");
//     } else {
//         println!("{seconds} seconds to blastoff...");
//         countdown(seconds - 1);
//     }
// }
// fn main() {
//     countdown(6);

//     countdown(5);

//     countdown(4);
// }

// ---- Coding Challenge ----
fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_iterative(number: i32) -> i32 {
    let mut product = 1;
    let mut count = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    product
}

fn factorial_recursive(number: i32) -> i32 {
    if  number == 1 {
        return 1;
    } else {
        return number * factorial_recursive(number - 1);
    }
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number("purple"));

    println!("{}", factorial_recursive(5));
    println!("{}", factorial_iterative(5));
}
