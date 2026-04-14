#[derive(PartialEq)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}

struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

fn main() {
    let a = Flight::new("New York", "London", "08:00");
    let b = BusTrip::new("Los Angeles", "Tokyo", "08:00");

    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == a);
    println!("{}", b == b);
}

// -------------------------------------------------------------- //

// #[derive(Clone, Debug)]
// struct Duration {
//     hours: u32,
//     minutes: u32,
//     seconds: u32,
// }

// impl Duration {
//     fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
//         Self {
//             hours,
//             minutes,
//             seconds,
//         }
//     }
// }

// impl Copy for Duration {}

// fn main() {
//     let one_hour = Duration::new(60, 0, 0);
//     let another_hour = one_hour;

//     println!("{:?}", one_hour);
// }

// -------------------------------------------------------------- //

// use std::clone::Clone;

// #[derive(Clone, Debug)]
// struct Appointment {
//     doctor: String,
//     start_time: String,
//     end_time: String,
// }

// impl Appointment {
//     fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
//         Self {
//             doctor: doctor.to_string(),
//             start_time: start_time.to_string(),
//             end_time: end_time.to_string(),
//         }
//     }
// }

// // impl Clone for Appointment {
// //     fn clone(&self) -> Self {
// //         println!("Cloning Appointment");

// //         Self {
// //             doctor: self.doctor.clone(),
// //             start_time: self.start_time.clone(),
// //             end_time: self.end_time.clone(),
// //         }
// //     }
// // }

// fn main() {
//     let morning_appt = Appointment::new("Dr. Andrews", "9:00AM", "10:00AM");
//     let replacement_appt = morning_appt.clone();
//     println!(
//         "{} is seeing the patient from {} to {}",
//         replacement_appt.doctor, replacement_appt.start_time, replacement_appt.end_time
//     );
//     println!("{morning_appt:?}");
// }

// -------------------------------------------------------------- //

// use std::fmt::{Debug, Display, Formatter, Result};
// use std::fs;
// use std::ops::Drop;

// enum AppleType {
//     RedDelicious,
//     GrannySmith,
// }

// impl Display for AppleType {
//     fn fmt(&self, formatter: &mut Formatter) -> Result {
//         match self {
//             AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
//             AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
//         }
//     }
// }

// impl Debug for AppleType {
//     fn fmt(&self, formatter: &mut Formatter) -> Result {
//         match self {
//             AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
//             AppleType::GrannySmith => write!(formatter, "AppleType::GrannySmith"),
//         }
//     }
// }

// struct Apple {
//     kind: AppleType,
//     price: f64,
// }

// impl Display for Apple {
//     fn fmt(&self, formatter: &mut Formatter) -> Result {
//         write!(formatter, "{} for {}", self.kind, self.price)
//     }
// }

// impl Debug for Apple {
//     fn fmt(&self, formatter: &mut Formatter) -> Result {
//         formatter
//             .debug_struct("** Apple **")
//             .field("Kind", &self.kind)
//             .field("Price", &self.price)
//             .finish()
//     }
// }

// impl Drop for Apple {
//     fn drop(&mut self) {
//         match fs::remove_file("apple.txt") {
//             Ok(_) => println!("Goodbye, my sweet apple"),
//             Err(error) => println!("Error cleaning up file: {error}"),
//         }
//     }
// }

// fn main() {
//     let lunch_snack = Apple {
//         kind: AppleType::GrannySmith,
//         price: 1.04,
//     };

//     let dinner_snack = Apple {
//         kind: AppleType::RedDelicious,
//         price: 1.15,
//     };

//     println!("{:?}", lunch_snack);
//     println!("{:?}", dinner_snack);
// }

// -------------------------------------------------------------- //

// trait Investment<T> {
//     fn amount(&self) -> T;

//     fn double_amount(&mut self);
// }
// trait Taxable: Investment<f64> {
//     const TAX_RATE: f64 = 0.25;

//     fn tax_bill(&self) -> f64 {
//         self.amount() * Self::TAX_RATE
//     }
// }

// #[derive(Debug)]
// struct Income {
//     amount: f64,
// }

// impl Investment<f64> for Income {
//     fn amount(&self) -> f64 {
//         self.amount
//     }

//     fn double_amount(&mut self) {
//         self.amount *= 2.0
//     }
// }

// impl Taxable for Income {}

// #[derive(Debug)]
// struct Bonus {
//     value: f64,
// }

// impl Investment<f64> for Bonus {
//     fn amount(&self) -> f64 {
//         self.value
//     }

//     fn double_amount(&mut self) {
//         self.value *= 2.0
//     }
// }

// impl Taxable for Bonus {
//     const TAX_RATE: f64 = 0.50;
// }

// #[derive(Debug)]
// struct QualityTime {
//     minutes: u32,
// }

// impl Investment<u32> for QualityTime {
//     fn amount(&self) -> u32 {
//         self.minutes
//     }

//     fn double_amount(&mut self) {
//         self.minutes *= 2
//     }
// }

// fn main() {
//     let mut income = Income { amount: 50000.50 };
//     let mut bonus = Bonus { value: 10000.23 };
//     let mut rust_programming_time = QualityTime { minutes: 120 };

//     income.double_amount();
//     bonus.double_amount();
//     rust_programming_time.double_amount();

//     println!("Total tax owed: ${:.2}", income.tax_bill());
//     println!("Bonus tax owed : ${:.2}", bonus.tax_bill());
//     println!("{:#?}", rust_programming_time.amount());
// }

// -------------------------------------------------------------- //

// use traits::logging::{Accommodation, AirBnB, Description, Hotel};
// use traits::utils;

// fn main() {
//     let mut hotel = Hotel::new(String::from("The Luxe"));
//     println!("{}", hotel.summarize());
//     hotel.book("Dana", 5);

//     let mut airbnb = AirBnB::new("Parker");
//     println!("{}", airbnb.get_description());
//     utils::book_for_one_night(&mut airbnb, "Dan");

//     utils::mix_and_match(&mut hotel, &mut airbnb, "Phil");
// }

// -------------------------------------------------------------- //
