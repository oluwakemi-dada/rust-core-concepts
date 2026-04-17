// fn main() {
//     let multiplier = 5;

//     // fn multiply_by(value: i32) -> i32 {
//     //     value * multiplier
//     // }

//     let multiply_by = |value: i32| -> i32 { return multiplier * value };

//     println!("{}", multiply_by(2));

//     let product = |a: i32, b: i32| -> i32 {
//         println!("Calculating product for you");
//         a * b
//     };

//     println!("{}", product(3, 9));
//     println!("{}", product(5, 8));
// }

// --------------------------------------------------- //

// fn main() {
//     let multiplier = 5;

//     let multiply_by = |value| value * multiplier;
//     println!("{}", multiply_by(3 as u8));

//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     let print_number = || println!("{:?}", numbers);
//     print_number();
//     print_number();
//     print_number();
//     println!("{:?}", numbers);
// }

// --------------------------------------------------- //

// fn main() {
//     let mut numbers = vec![4, 8, 15, 16, 23, 42];
//     let mut add_number = || numbers.push(100);
//     add_number();
//     // println!("{:?}", numbers); // not possible
//     add_number();
//     add_number();
//     println!("{:?}", numbers);
// }

// --------------------------------------------------- //

// fn main() {
//     let number = 13;
//     let capture_number = || number;

//     let a = capture_number();
//     let b = capture_number();
//     println!("{a} {b} {number}");

//     let first_name = String::from("Alice");
//     let capture_string = || {
//         let person = first_name;
//         println!("{person}");
//     };
//     capture_string();
//     // capture_string(); cannot be called more than once
// }

// --------------------------------------------------- //

// fn main() {
//     let first_name = String::from("Alice");
//     let last_name = String::from("Wonder");
//     let capture_string = move || {
//         println!("{first_name} {last_name}");
//     };
//     capture_string();
//     capture_string();
//     capture_string();

//     // println!("{first_name}");
//     // println!("{last_name}");
// }

// --------------------------------------------------- //

// fn main() {
//     let option = Some("Salami");
//     let closure = || "Pizza";
//     let food = option.unwrap_or_else(closure);
//     println!("{food}");

//     let option: Option<&str> = None;
//     let pizza_fan = false;
//     let closure = || if pizza_fan { "Pizza" } else { "Hot Pockets" };
//     let food = option.unwrap_or_else(closure);
//     println!("{food}");
// }

// --------------------------------------------------- //

// use std::io::stdin;

// #[derive(Debug)]
// struct Vault {
//     password: String,
//     treasure: String,
// }

// impl Vault {
//     fn unlock<F>(self, procedure: F) -> Option<String>
//     where
//         F: FnOnce() -> String,
//     {
//         let user_password = procedure();
//         if user_password == self.password {
//             Some(self.treasure)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     let vault = Vault {
//         password: String::from("topsecret"),
//         treasure: String::from("Gold"),
//     };

//     let hack = || {
//         let mut user_input = String::new();
//         println!("Please provide a password to crack the vault");
//         stdin().read_line(&mut user_input);
//         user_input.trim().to_string()
//     };

//     let extraction = vault.unlock(hack);

//     println!("{:?}", extraction);
// }

// --------------------------------------------------- //

// fn main() {
//     let mut game_console = String::from("PlayStation");
//     let mut deleted_characters = String::new();

//     let closure = |character| {
//         let is_not_a = character != 'a';
//         if is_not_a {
//             true
//         } else {
//             deleted_characters.push(character);
//             false
//         }
//     };
//     game_console.retain(closure);
//     println!("{game_console}");
//     println!("{deleted_characters}");
// }

// --------------------------------------------------- //

// #[derive(Debug)]
// struct Location {
//     name: String,
//     treasures: u32,
// }

// struct Map<'a> {
//     locations: &'a [Location],
// }

// impl<'a> Map<'a> {
//     fn explore<F>(&self, mut action: F)
//     where
//         F: FnMut(&Location),
//     {
//         let final_index = self.locations.len() - 1;
//         let mut current_index = 0;
//         while current_index <= final_index {
//             let current_location = &self.locations[current_index];
//             action(current_location);
//             current_index += 1;
//         }
//     }
// }

// fn main() {
//     let locations = [
//         Location {
//             name: String::from("Enchanted Forest"),
//             treasures: 5,
//         },
//         Location {
//             name: String::from("Mystic Mountain"),
//             treasures: 10,
//         },
//     ];
//     let map = Map {
//         locations: &locations,
//     };
//     let mut total_treasures = 0;

//     map.explore(|location| {
//         total_treasures += location.treasures;
//     });

//     println!("Total treasures collected: {}", total_treasures);

//     let mut location_names: Vec<String> = Vec::new();

//     map.explore(|location| {
//         location_names.push(location.name.clone());
//     });
//     println!("{location_names:?}");
// }

// --------------------------------------------------- //

// fn execute_thrice<F>(mut procedure: F)
// where
//     F: FnMut(),
// {
//     procedure();
//     procedure();
//     procedure();
// }

// fn main() {
//     let mut bosses = vec!["Boris"];
//     let closure = || {
//         bosses.push("Alexandra");
//     };
//     execute_thrice(closure);
// }

// --------------------------------------------------- //

fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(),
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("Hello chocolate cake");
}

fn main() {
    execute_thrice(bake_cake);

    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new);
    println!("{:?}", collection);
}

