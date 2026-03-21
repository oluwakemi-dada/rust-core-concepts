// ---- Coding Challenge ----
#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }

        if self.reservations < 12 {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Strip Steak"),
            })
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err(String::from("Sorry, we have a mice problem"));
        }

        if address.is_empty() {
            return Err(String::from("No delivery address specified"));
        }

        Ok(Food {
            name: String::from("Burger"),
        })
    }
}
fn main() {
    let marios = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    println!("{:?}", marios.chef_special());
    println!("{:?}", marios.deliver_burger("123 Elm Street"));

    let angelos = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("{:?}", angelos.chef_special());
    println!("{:?}", angelos.deliver_burger(""));
    println!("{:?}", angelos.deliver_burger("123 Elm Street"));
}

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
// fn main() {
//     let present_value = Some(13);
//     let missing_value: Option<bool> = None;

//     println!("{}", present_value.unwrap_or(0));
//     println!("{}", missing_value.unwrap_or(true));
// }

// ---- Building Option from Scratch ----
// #[derive(Debug, Copy, Clone)]
// enum MyOption {
//     Some(i32),
//     None,
// }

// impl MyOption {
//     fn unwrap(self) -> i32 {
//         match self {
//             MyOption::Some(value) => value,
//             MyOption::None => panic!("Uh oh"),
//         }
//     }

//     fn unwrap_or(self, fallback_value: i32) -> i32 {
//         match self {
//             MyOption::Some(value) => value,
//             MyOption::None => fallback_value,
//         }
//     }
// }

// fn main() {
//     let some_option = MyOption::Some(100);
//     println!("{}", some_option.unwrap());
//     println!("{}", some_option.unwrap_or(25));

//     let none_option = MyOption::None;
//     // println!("{}", none_option.unwrap());
//     println!("{}", none_option.unwrap_or(25));
// }

// ---- The Result Enum ----
// fn main() {
//     let ok: Result<i32, &str> = Ok(5);
//     println!("{ok:?}");
//     let disaster: Result<i32, &str> = Err("Something went wrong");
//     println!("{:?}", disaster);
// }

// ---- The Result Enum Example ----
// fn main() {
//     let text = "50";
//     let text_as_number: Result<_, _> = text.parse::<i32>();
//     println!("{:?}", text_as_number);

//     let text = "Alabama";
//     let text_as_number: Result<_, _> = text.parse::<i32>();
//     println!("{:?}", text_as_number);
// }

// ---- Returning a Result Enum from a Function | Result Methods ----
// fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err("Cannot divide by zero".to_string())
//     } else {
//         Ok(numerator / denominator)
//     }
// }

// fn main() {
//     let result = divide(10.0, 0.0);

//     match result {
//         Ok(calculation) => println!("Result: {}", calculation),
//         Err(message) => println!("Error: {}", message),
//     }
// }

// ---- Result Methods ----
// fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err("Cannot divide by zero".to_string())
//     } else {
//         Ok(numerator / denominator)
//     }
// }

// fn main() {
//     let result = divide(10.0, 2.0);

//   println!("{}", result.is_ok());
//   println!("{}", result.is_err());
// }

// ---- Nuances of unwrap Method on Result ----
// fn operation(great_success: bool) -> Result<&'static str, &'static str> {
//     if great_success {
//         Ok("Success")
//     } else {
//         Err("Error")
//     }
// }
// fn main() {
//     let my_result = operation(true);

//     let content = match my_result {
//         Ok(message) => message,
//         Err(error) => error,
//     };

//     println!("{}", my_result.unwrap())
// }

// ---- The while let Construct ----
// fn main() {
//     let mut sauces = vec!["Mayonnaise", "Ketchup", "Ranch"];

//     while let Some(sauce) = sauces.pop() {
//         println!("The next sauce is {sauce}");
//     }
// }
