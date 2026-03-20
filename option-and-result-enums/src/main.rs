// ---- The Option Enum ----
// fn main() {
//     let a = Option::Some(5);
//     let b = Option::Some("hello");
//     let c = Option::Some(true);

//     let a: Option<i8> = Option::Some(5);
//     let b = Option::<i16>::Some(5);

//     let d: Option<&str> = Option::None;
// }

// ---- The Option Enum Example | The unwrap and expect Methods ----
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Bass"),
//     ];

//     let bass = musical_instruments.get(2);
//     println!("{:?}", bass);
//     let valid_instrument = bass.expect("Unable to retrieve element");
//     println!("{valid_instrument}");

//     let invalid_instrument = musical_instruments.get(10);
//     println!("{:?}", invalid_instrument);
//     // println!("{}", invalid_instrument.unwrap());
//     println!(
//         "{}",
//         invalid_instrument.expect("Unable to retrieve element")
//     );
// }

// ---- The match Keyword with Option Enum ----
// fn main() {
//     let musical_instruments = [
//         String::from("Guitar"),
//         String::from("Drums"),
//         String::from("Bass"),
//     ];

//     let bass = musical_instruments.get(2);

//     play(bass); // copy trait implemented here

//     let invalid_instrument = musical_instruments.get(10);

//     play(invalid_instrument); // copy trait implemented here
// }

// fn play(instrument_option: Option<&String>) {
//     match instrument_option {
//         Option::Some(instrument_option) => println!("Playing the {instrument_option}"),
//         Option::None => println!("Singing with my voice"),
//     }
// }

// ---- Returning an Option Enum from a Function ----
// fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
//     if item_is_in_system && item_is_in_stock {
//         Option::Some(true)
//     } else if item_is_in_system {
//         Option::Some(false)
//     } else {
//         Option::None
//     }
// }

// fn main() {
//     let availability = is_item_in_stock(true, false);

//     match availability {
//         // Option::Some(value) => println!("Item is available: {value}"),
//         Option::Some(true) => println!("Yes, the item is available"),
//         Option::Some(false) => println!("No, the item is not in stock"),
//         Option::None => println!("Your item doesn't exist in our system"),
//     }
// }

// ---- Top-Level Option Variants ----
// fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
//     if item_is_in_system && item_is_in_stock {
//         Some(true)
//     } else if item_is_in_system {
//         Some(false)
//     } else {
//         None
//     }
// }

// fn main() {
//     let availability = is_item_in_stock(true, false);

//     match availability {
//         // Option::Some(value) => println!("Item is available: {value}"),
//         Some(true) => println!("Yes, the item is available"),
//         Some(false) => println!("No, the item is not in stock"),
//         None => println!("Your item doesn't exist in our system"),
//     }
// }

// ---- The unwrap_or Method ----
fn main() {
    let present_value = Some(13);
    let missing_value: Option<bool> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(true));
}
