// ---- Coding Challenge ----
#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}
fn main() {
    let message = ChatMessage {
        content: "Hola!!!",
        time: String::from("2026-01-12"),
    };

    let notification = ChatMessage {
        content: String::from("What's your favorite drink?"),
        time: String::from("2026-02-12"),
    };

    let audio = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("2026-03-12"),
    };

    // message.consume_entertainment();
    // notification.consume_entertainment();
    audio.consume_entertainment();

    println!("{}", message.retrieve_time());
    println!("{}", notification.retrieve_time());
    println!("{}", audio.retrieve_time());
}

// ---- Intro to Generics ----
// #[derive(Debug)]
// struct DeliSandwich {}
// fn main() {
//     println!("{}", identity::<i32>(5));
//     println!("{}", identity::<i8>(5));
//     println!("{}", identity::<u32>(5));
//     println!("{}", identity::<f64>(13.5));
//     println!("{}", identity::<&str>("Hello"));
//     println!("{}", identity::<String>(String::from("Hi there!")));
//     println!("{}", identity::<bool>(true));
//     println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));
// }

// fn identity<T>(value: T) -> T {
//     value
// }

// ---- Multiple Generics ----
// fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
//     (first, second)
// }

// fn main() {
//     make_tuple("Hello", 5);
//     make_tuple(13, 5);
//     make_tuple(true, 3.5);
//     make_tuple(true, false);
// }

// ---- Generics in Structs ----
// #[derive(Debug)]
// struct TresureChest<T> {
//     captain: String,
//     treasure: T,
// }

// impl TresureChest<String> {
//     fn clean_treasure(&mut self) {
//         self.treasure = self.treasure.trim().to_string();
//     }
// }

// impl TresureChest<[&str; 3]> {
//     fn amount_of_treasure(&self) -> usize {
//         self.treasure.len()
//     }
// }

// impl<T> TresureChest<T> {
//     fn capital_captain(&self) -> String {
//         self.captain.to_uppercase()
//     }
// }

// fn main() {
//     let gold_chest = TresureChest {
//         captain: String::from("Firebeard"),
//         treasure: "Gold",
//     };
//     println!("{}", gold_chest.capital_captain());

//     let mut silver_chest = TresureChest {
//         captain: String::from("Bloodsail"),
//         treasure: String::from("     Silver"),
//     };
//     silver_chest.clean_treasure();
//     println!("{}", silver_chest.capital_captain());

//     let special_chest = TresureChest {
//         captain: String::from("Bootyplunder"),
//         treasure: ["Gold", "Silver", "Platinum"],
//     };
//     println!("{}", special_chest.amount_of_treasure());
//     println!("{}", special_chest.capital_captain());
// }

// ---- Generics in Enums ----
// #[derive(Debug)]
// enum Cheesestick<T> {
//     Plain,
//     Topping(T),
// }
// fn main() {
//     let mushroom = Cheesestick::Topping("Mushroom");
//     let onions = Cheesestick::Topping("onions".to_string());
//     let topping = "bacon".to_string();
//     let bacon = Cheesestick::Topping(&topping);

//     let mut plain: Cheesestick<String> = Cheesestick::Plain;
//     plain = Cheesestick::Topping("sausage".to_string());
// }
