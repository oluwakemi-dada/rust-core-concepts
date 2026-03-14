// ---- Define a Struct | Struct instance | Access Struct Fields ----
// fn main() {
//     struct Coffee {
//         name: String,
//         price: f64,
//         is_hot: bool,
//     }

//     let mocha = Coffee {
//         name: String::from("Mocha"),
//         price: 4.99,
//         is_hot: false,
//     };

//     println!(
//         "My {} this morning cost {}. It is {} that it was hot",
//         mocha.name, mocha.price, mocha.is_hot
//     );

//     let favorite_coffee = mocha.name;
//     println!("{favorite_coffee}");
//     // println!("{}", mocha.name)
// }

// ---- Overwrite Struct Fields ----
// fn main() {
//     struct Coffee {
//         name: String,
//         price: f64,
//         is_hot: bool,
//     }

//     let mut beverage = Coffee {
//         name: String::from("Mocha"),
//         price: 4.99,
//         is_hot: false,
//     };

//     beverage.name = String::from("Caramel Macchiato");
//     beverage.price = 6.99;
//     beverage.is_hot = true;

//     println!(
//         "My {} this morning cost {}. It is {} that it was hot",
//         beverage.name, beverage.price, beverage.is_hot
//     );
// }

// ---- Create Structs in a Function | Struct Field Initialization Shorthand Syntax ----
// struct Coffee {
//     name: String,
//     price: f64,
//     is_hot: bool,
// }

// fn main() {
//     let name = String::from("Latte");
//     let coffee = make_coffee(name, 4.99, true);
//     println!(
//         "My {} this morning cost {}. It is {} that it was hot.",
//         coffee.name, coffee.price, coffee.is_hot
//     );
//     // println!("{name}");

//     let name = String::from("Latte");
//     let price = 3.99;
//     let is_hot = false;

//     let latte = Coffee {
//         name,
//         price,
//         is_hot,
//     };
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// ---- Struct Update Syntax ----
// struct Coffee {
//     name: String,
//     price: f64,
//     is_hot: bool,
// }

// fn main() {
//     let mocha = make_coffee(String::from("Mocha"), 4.99, true);

//     let caramel_macchiato = Coffee {
//         name: mocha.name.clone(),
//         ..mocha
//     };

//     println!("{}", caramel_macchiato.name);
//     println!("{}", mocha.name);
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// ---- Passing Structs into a Function ----
// struct Coffee {
//     name: String,
//     price: f64,
//     is_hot: bool,
// }

// fn main() {
//     let mut mocha = make_coffee(String::from("Mocha"), 4.99, true);
//     drink_coffee(&mut mocha);

//     println!("{}", mocha.price)
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// fn drink_coffee(coffee: &mut Coffee) {
//     println!("Drinking my delicious {}", coffee.name);
//     coffee.is_hot = false;
//     coffee.price = 10.99;
// }

// ---- Deriving Debug Trait for Struct ----
// #[derive(Debug)]
// struct Coffee {
//     name: String,
//     price: f64,
//     is_hot: bool,
// }

// fn main() {
//     let mocha = make_coffee(String::from("Mocha"), 4.99, true);

//     let values = ["hello", "world"];

//     println!("{:?}", mocha);
//     println!("{:#?}", mocha);
// }

// fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
//     Coffee {
//         name,
//         price,
//         is_hot,
//     }
// }

// fn drink_coffee(coffee: &mut Coffee) {
//     println!("Drinking my delicious {}", coffee.name);
//     coffee.is_hot = false;
//     coffee.price = 10.99;
// }

// ---- Derive Struct Methods ----
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(self: Self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration_secs);
    }
}

fn main() {
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info(); // song transfers ownership to display_song_info as "self" indirectly
    // println!("{}", song.title);
}
