use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(formatter, "AppleType::GrannySmith"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        formatter
            .debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye, my sweet apple"),
            Err(error) => println!("Error cleaning up file: {error}"),
        }
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };

    println!("{:?}", lunch_snack);
    println!("{:?}", dinner_snack);
}

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
