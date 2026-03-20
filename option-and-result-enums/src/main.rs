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
fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(2);
    println!("{:?}", bass);
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{valid_instrument}");

    let invalid_instrument = musical_instruments.get(10);
    println!("{:?}", invalid_instrument);
    // println!("{}", invalid_instrument.unwrap());
    println!(
        "{}",
        invalid_instrument.expect("Unable to retrieve element")
    );
}
