// ---- Intro to Enums ----
// #[derive(Debug)]
// enum CardSuit {
//     Hearts,
//     Diamond,
//     Spades,
//     Clubs,
// }

// struct Card {
//     rank: String,
//     suit: CardSuit,
// }

// fn main() {
//     let first_card = CardSuit::Hearts;
//     let mut second_card = CardSuit::Spades;
//     second_card = CardSuit::Clubs;

//     println!("{:?}", second_card);

//     let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
//     let card_suits = (CardSuit::Hearts, CardSuit::Spades);
// }

// ---- Enum with Associated Values ----
// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     PayPal(String, String),
// }
// fn main() {
//     let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0012-3456"));

//     my_payment_method =
//         PaymentMethodType::PayPal(String::from("john@email.com"), String::from("password"));

//     println!("{:?}", my_payment_method)
// }

// ---- Struct Variants ----
// #[derive(Debug)]
// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     PayPal { username: String, password: String },
//     Cash,
// }

// fn main() {
//     let visa = PaymentMethodType::CreditCard(String::from("1234-5678"));

//     let paypal = PaymentMethodType::PayPal {
//         username: String::from("bob@gmail.com"),
//         password: String::from("password"),
//     };

//     println!("{:?}", visa);
//     println!("{:?}", paypal);
// }

// ---- Nesting Enums in Enums ----
// #[derive(Debug)]
// enum Beans {
//     Pinto,
//     Black,
// }

// #[derive(Debug)]
// enum Meat {
//     Chicken,
//     Steak,
// }

// #[derive(Debug)]
// enum RestaurantItem {
//     Burrito { meat: Meat, beans: Beans },
//     Bowl { meat: Meat, beans: Beans },
//     VeganPlate,
// }

// fn main() {
//     let lunch = RestaurantItem::Burrito {
//         meat: Meat::Steak,
//         beans: Beans::Pinto,
//     };
//     let dinner = RestaurantItem::Bowl {
//         meat: Meat::Chicken,
//         beans: Beans::Black,
//     };
//     let abandoned_meal = RestaurantItem::VeganPlate;

//     println!("Lunch was {lunch:?} and dinner was {dinner:?}");
//     println!("Nobody ate {abandoned_meal:?}");
// }

// ---- The match Keyword I ----
// #[derive(Debug)]
// enum OperatingSystem {
//     Windows,
//     MacOS,
//     Linux,
// }

// fn main() {
//     let my_computer = OperatingSystem::MacOS;
//     let age = years_since_release(my_computer);
//     println!("My computer's operating system is {age} years old");

//     let dads_computer = OperatingSystem::Windows;
//     let age = years_since_release(dads_computer);
//     println!("My dad's computer is {age} years old");
// }

// fn years_since_release(os: OperatingSystem) -> u32 {
//     match os {
//         OperatingSystem::Windows => {
//             println!("Quite an old operating system");
//             39
//         }
//         OperatingSystem::MacOS => 23,
//         OperatingSystem::Linux => 34,
//     }
// }

// ---- Methods on Enums ----
// #[derive(Debug)]
// enum LaundryCycle {
//     Cold,
//     Hot { temperature: u32 },
//     Delicate(String),
// }

// impl LaundryCycle {
//     fn wash_laundry(&self) {
//         match self {
//             LaundryCycle::Cold => {
//                 println!("Running the laundry with cold temperature")
//             },
//             LaundryCycle::Hot { temperature } => {
//                 println!("Running the laundry with a temperature of {temperature}")
//             },
//             LaundryCycle::Delicate(fabric_type) => {
//                 println!("Running the laundry with delicate cycle for {fabric_type}")
//             }
//         }
//     }
// }

// fn main() {
//     LaundryCycle::Cold.wash_laundry();

//     let hot_cycle = LaundryCycle::Hot { temperature: 100 };
//     hot_cycle.wash_laundry();

//     let delicate_cycle = LaundryCycle::Delicate(String::from("Silk"));
//     delicate_cycle.wash_laundry();
// }

// ---- The match Keyword - Catching Multiple Values ----
#[derive(Debug)]

enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered")
            }
            order_status => {
                println!("Your item is {order_status:?}")
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Shipped.check();
}
