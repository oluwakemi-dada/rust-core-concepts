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
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn display_song_info(self: Self) {
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {}", self.duration_secs);
//     }
// }

// fn main() {
//     let song = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     song.display_song_info(); // song transfers ownership to display_song_info as "self" indirectly
//     // println!("{}", song.title);
// }

// ---- self Parameter as Mutable Struct Instance ----
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     // Immutable struct value (self parameter takes ownership)
//     fn display_song_info(self) {
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {}", self.duration_secs);
//     }

//     // Mutable struct value (self parameter takes ownership, has permission to mutate)
//     fn double_length(mut self) {
//         self.duration_secs = self.duration_secs * 2;
//         println!("{:#?}", self);
//     }
// }

// fn main() {
//     let song = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     song.double_length(); // we can no longer call another method because ownership is lost here
// }

// ---- self Parameter as Immutable and Mutable References to Struct Instance ----
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     // Immutable struct value (self parameter takes ownership)
//     // -> Immutable reference to the struct instance (no ownership moved)
//     fn display_song_info(&self) {
//         // self: &Self
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {}", self.duration_secs);
//     }

//     // Mutable struct value (self parameter takes ownership, has permission to mutate)
//     // -> Mutable reference to the struct instance (no ownership moved, has permission to mutate)
//     fn double_length(&mut self) {
//         // self: &mut Self
//         self.duration_secs = self.duration_secs * 2;
//     }
// }

// fn main() {
//     let mut song = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     song.display_song_info(); // &song
//     song.double_length();
//     song.display_song_info();
// }

// ---- Methods with Multiple Parameters ----
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Release Year: {}", self.release_year);
//         println!("Duration: {}", self.duration_secs);
//     }

//     fn double_length(&mut self) {
//         self.duration_secs = self.duration_secs * 2;
//     }

//     fn is_longer_than(&self, other: &Self) -> bool {
//         self.duration_secs > other.duration_secs
//     }
// }

// fn main() {
//     let blank_space = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     let all_too_well = TaylorSwiftSong {
//         title: String::from("All Too Well"),
//         release_year: 2012,
//         duration_secs: 327,
//     };

//     if blank_space.is_longer_than(&all_too_well) {
//         println!(
//             "{} is longer than {}",
//             blank_space.title, all_too_well.title
//         )
//     } else {
//         println!(
//             "{} is shorter than or equal to {}",
//             blank_space.title, all_too_well.title,
//         )
//     }
// }

// ---- Calling Methods from Other Methods ----
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Years Since Release: {}", self.years_since_release());
//         println!("Duration: {}", self.duration_secs);
//     }

//     fn double_length(&mut self) {
//         self.duration_secs = self.duration_secs * 2;
//     }

//     fn is_longer_than(&self, other: &Self) -> bool {
//         self.duration_secs > other.duration_secs
//     }

//     fn years_since_release(&self) -> u32 {
//         2026 - self.release_year
//     }
// }

// fn main() {
//     let blank_space = TaylorSwiftSong {
//         title: String::from("Blank Space"),
//         release_year: 2014,
//         duration_secs: 231,
//     };

//     blank_space.display_song_info();
// }

// ---- Associated Functions ----
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
//         Self {
//             title,
//             release_year,
//             duration_secs,
//         }
//     }

//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Years Since Release: {}", self.years_since_release());
//         println!("Duration: {}", self.duration_secs);
//     }

//     fn double_length(&mut self) {
//         self.duration_secs = self.duration_secs * 2;
//     }

//     fn is_longer_than(&self, other: &Self) -> bool {
//         self.duration_secs > other.duration_secs
//     }

//     fn years_since_release(&self) -> u32 {
//         2026 - self.release_year
//     }
// }

// fn main() {
//     let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);

//     blank_space.display_song_info();
// }

// ---- Multiple impl Blocks ----
// #[derive(Debug)]
// struct TaylorSwiftSong {
//     title: String,
//     release_year: u32,
//     duration_secs: u32,
// }

// impl TaylorSwiftSong {
//     fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
//         Self {
//             title,
//             release_year,
//             duration_secs,
//         }
//     }
// }

// impl TaylorSwiftSong {
//     fn display_song_info(&self) {
//         println!("Title: {}", self.title);
//         println!("Years Since Release: {}", self.years_since_release());
//         println!("Duration: {}", self.duration_secs);
//     }

//     fn double_length(&mut self) {
//         self.duration_secs = self.duration_secs * 2;
//     }

//     fn is_longer_than(&self, other: &Self) -> bool {
//         self.duration_secs > other.duration_secs
//     }

//     fn years_since_release(&self) -> u32 {
//         2026 - self.release_year
//     }
// }

// fn main() {
//     let blank_space = TaylorSwiftSong::new(String::from("Blank Space"), 2014, 231);

//     blank_space.display_song_info();
// }

// ---- Builder Pattern ----
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}
fn main() {
    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(4);

    println!("Stats: {:#?}", computer);
}
