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
#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String },
    Cash,
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("1234-5678"));

    let paypal = PaymentMethodType::PayPal {
        username: String::from("bob@gmail.com"),
        password: String::from("password"),
    };

    println!("{:?}", visa);
    println!("{:?}", paypal);
}
