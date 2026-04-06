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

// ----- Common String Methods (trim, casing, replace, split)-----
fn main() {
    let mut music_genres = "    Rock, Metal, Country, Rap  ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{genres:?}");
}
