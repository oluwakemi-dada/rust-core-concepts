use std::io;

// ----- Coding Challenge -----
fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();

    let input = io::stdin();

    println!("What is your first name?");
    input
        .read_line(&mut first_name)
        .expect("Failed to collect first name");

    println!("What is your last name?");
    input
        .read_line(&mut last_name)
        .expect("Failed to collect last name");

    format!("{} {}", first_name.trim(), last_name.trim())
}

fn main() {
    let mut amount = String::from("40");
    make_money(&mut amount);
    println!("{amount}");

    let banana = trim_and_capitalize(".      banana   ");
    println!("{banana}");

    let collection = elements("Gold!Silver!Platinum");
    println!("{collection:?}");

    let full_name = get_identity();
    println!("{full_name}")
}

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
// fn main() {
//     let first_name = String::from("Sylvester");
//     let last_name = String::from("Stallone");

//     let full_name = first_name + &last_name;
//     // let full_name = first_name.clone() + &last_name;
//     println!("{full_name}");
//     println!("{first_name}")
// }

// ----- The format! Macro -----
// fn main() {
//     let first_name = String::from("Sylvester");
//     let last_name = String::from("Stallone");

//     let icon = format!("{first_name} {last_name}");
//     println!("{icon}");
//     println!("{first_name}");
//     println!("{last_name}");
// }

// ----- Common String Methods (trim, casing, replace, split) -----
// fn main() {
//     let mut music_genres = "    Rock, Metal, Country, Rap  ";
//     println!("{}", music_genres.trim());
//     println!("{}", music_genres.trim_start());
//     println!("{}", music_genres.trim_end());

//     music_genres = music_genres.trim();
//     println!("{}", music_genres.to_uppercase());
//     println!("{}", music_genres.to_lowercase());

//     println!("{}", music_genres.replace("a", "@"));

//     let genres: Vec<&str> = music_genres.split(", ").collect();
//     println!("{genres:?}");
// }

// ----- Collecting User Input with read_line Method -----
// use std::io;

// fn main() {
//     let mut name = String::new();
//     println!("What is your name?");
//     match io::stdin().read_line(&mut name) {
//         Ok(_) => println!("Hello, {}", name.trim()),
//         Err(error) => println!("There was an error: {error}"),
//     }
// }
