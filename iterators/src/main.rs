#![allow(unused, dead_code)]
use std::{collections::HashMap, env};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let blender_orders = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();
    // println!("{blender_orders:#?}");

    let microwave_count = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum::<u32>();
    // println!("{microwave_count}");

    // tackled 2 cases, non integer and no value
    let user_quantity = env::args()
        .skip(1)
        .take(1)
        .map(|quantity| quantity.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);

    let orders_by_quantity = orders
        .iter()
        .filter(|order| order.quantity >= user_quantity)
        .collect::<Vec<&CustomerOrder>>();
    // println!("{orders_by_quantity:#?}");

    let product_quantities = orders.iter().filter(|order| order.shipped == false).fold(
        HashMap::new(),
        |mut data, order: &CustomerOrder| {
            let entry = data.entry(&order.product).or_insert(0);
            *entry += order.quantity;
            data
        },
    );
    // println!("{product_quantities:#?}");

    if let Some(order) = orders.iter_mut().find(|order| order.shipped == false) {
        order.shipped = true;
    }
    // println!("{orders:#?}");

    let mut customers = orders
        .into_iter()
        .zip(customer_ids_by_order)
        .fold(HashMap::new(), |mut ids_to_orders, (order, customer_id)| {
            let mut orders = ids_to_orders.entry(customer_id).or_insert(vec![]);
            orders.push(order);
            ids_to_orders
        })
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect::<Vec<Customer>>();

    customers.sort_by_key(|customer| customer.id);
    println!("{customers:#?}");
}

// --------------------------------------------------------- //

// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     for number in numbers {
//         println!("{number}")
//     }
// }

// --------------------------------------------------------- //

// use std::{collections::HashMap, hash::Hash};
// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     let my_iterator = my_vector.into_iter();

//     let my_vector = vec![false, true, false, true];
//     let my_iterator = my_vector.into_iter();

//     let mut my_hashmap = HashMap::new();
//     my_hashmap.insert("CBS", 2);
//     let my_iterator  = my_hashmap.into_iter();
// }

// --------------------------------------------------------- //

// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     let mut my_iterator = my_vector.into_iter();

//     println!("{:?}", my_iterator);

//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());
//     println!("{:?}", my_iterator.next());

//     println!("{:?}", my_iterator);
// }

// --------------------------------------------------------- //

// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];
//     // let  my_iterator = my_vector.into_iter();

//     for number in my_vector {
//       println!("{number}");
//     }

//     // println!("{:?}", my_vector);
// }

// --------------------------------------------------------- //

// fn main() {
//     let my_vector = vec![4, 8, 15, 16, 23, 42];

//     for number in &my_vector {
//         println!("{number}");
//     }

//     println!("{:?}", my_vector);

//     let cities = vec![String::from("Phoenix"), String::from("Dallas")];

//     // cities.iter()
//     for city in &cities {
//         println!("{}", city)
//     }

//     println!("{:?}", cities);
// }

// --------------------------------------------------------- //

// fn main() {
//     let mut flavors = [
//         String::from("Chocolate"),
//         String::from("Vanilla"),
//         String::from("Strawberry"),
//     ];

//     // let iterator = flavors.iter_mut();
//     // flavors.iter_mut()

//     for flavor in &mut flavors {
//         flavor.push_str(" Ice Cream");
//     }

//     println!("{:?}", flavors);

//     let mut school_grades = [85, 90, 72, 92];

//     for grades in &mut school_grades {
//         *grades -= 2;
//     }

//     println!("{:?}", school_grades);
// }

// --------------------------------------------------------- //

// use std::collections::HashMap;

// fn main() {
//     let mut todos: HashMap<_, _> = HashMap::new();
//     todos.insert("Pick up groceries", false);
//     todos.insert("Study Rust", true);
//     todos.insert("Sleep", false);

//     for (_, completion_status) in &mut todos {
//        *completion_status = true;
//     }

//     println!("{todos:?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let seafood = String::from("Oyster🦪");

//     // for byte in seafood.bytes() {
//     //     println!("{byte}/");
//     // }

//     // for character in seafood.chars() {
//     //     println!("{character}/");
//     // }

//     // println!("{seafood}");

//     println!("{:?}", seafood.bytes().len());
//     println!("{:?}", seafood.chars().count());
// }

// --------------------------------------------------------- //

// use std::collections::HashMap;

// fn count_words(text: &str) -> HashMap<char, u32> {
//     let words = text.split_whitespace();
//     let mut counts = HashMap::new();

//     // for word in words {
//     //     for character in word.chars() {
//     //         let count = counts.entry(character).or_insert(0);
//     //         *count += 1
//     //     }
//     // }

//     words.for_each(|word| {
//         word.chars().for_each(|character| {
//             let count = counts.entry(character).or_insert(0);
//             *count += 1
//         })
//     });

//     counts
// }

// fn main() {
//     println!(
//         "{:#?}",
//         count_words("Sally sells sea shells by the sea shore")
//     );
// }

// --------------------------------------------------------- //

// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
//         println!("Square: {number}");
//     }

//     // println!("{:?}", squares);
// }

// --------------------------------------------------------- //

// use std::{collections::HashSet, hash::Hash};

// fn main() {
//     let numbers = vec![4, 8, 8, 15, 16, 23, 42];
//     let squares = numbers
//         .iter()
//         .map(|number: &i32| number.pow(2))
//         .collect::<HashSet<i32>>();
//     println!("{:?}", squares);
//     println!("{:?}", numbers);
// }

// --------------------------------------------------------- //

// fn main() {
//     let names = [
//         String::from("Jimmy"),
//         String::from("Cleveland"),
//         String::from("Boris"),
//     ];

//     let name_lengths: Vec<usize> = names
//         .iter()
//         .map(|name| name.to_lowercase())
//         .map(|name| name.replace("i", "@@"))
//         .map(|name| name.len())
//         .collect();

//     println!("{:?}", name_lengths);
// }

// --------------------------------------------------------- //

// fn main() {
//     let numbers = [10, 13, 23, 2, 8, 9, 6];

//     let evens: Vec<i32> = numbers
//         .iter()
//         .filter(|number| *number % 2 == 0)
//         .copied()
//         .collect();
//     println!("{:?}", evens);
//     println!("{:?}", numbers);

//     let first_even = numbers.into_iter().find(|number| number % 2 == 0);
//     println!("{:?}", first_even);

//     let first_odd = numbers.into_iter().find(|number| number % 2 != 0);
//     println!("{:?}", first_odd);

//     let nothing = numbers.into_iter().find(|number| *number > 100);
//     println!("{:?}", nothing);

//     let last_even = numbers.iter().rfind(|number| *number % 2 == 0);
//     println!("{last_even:?}");

//     let last_odd = numbers.iter().rfind(|number| *number % 2 != 0);
//     println!("{last_odd:?}");
// }

// --------------------------------------------------------- //

// #[derive(Debug, PartialEq, Eq)]
// enum ChannelType {
//     Comedy,
//     News,
//     ProgrammingTutorials,
// }

// #[derive(Debug)]
// struct TVChannel {
//     name: String,
//     channel_type: ChannelType,
// }

// fn main() {
//     let channels = [
//         TVChannel {
//             name: String::from("CBS"),
//             channel_type: ChannelType::Comedy,
//         },
//         TVChannel {
//             name: String::from("RustLive"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//         TVChannel {
//             name: String::from("NBC"),
//             channel_type: ChannelType::News,
//         },
//         TVChannel {
//             name: String::from("RustTV"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//     ];

//     let good_channels: Vec<String> = channels
//         .iter()
//         .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
//         .map(|channel| channel.name.clone())
//         .collect();

//     println!("{:#?}", good_channels);

//     let good_channel = channels
//         .iter()
//         .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

//     match good_channel {
//         Some(channel) => println!("Great choice to watch {channel:#?}"),
//         None => println!("There was no Rust programming on the TV (literally and metaphorically)."),
//     }
// }

// --------------------------------------------------------- //

// #[derive(Debug, PartialEq, Eq)]
// enum ChannelType {
//     Comedy,
//     News,
//     ProgrammingTutorials,
// }

// #[derive(Debug)]
// struct TVChannel {
//     name: String,
//     channel_type: ChannelType,
// }

// fn main() {
//     let channels = [
//         TVChannel {
//             name: String::from("CBS"),
//             channel_type: ChannelType::Comedy,
//         },
//         TVChannel {
//             name: String::from("RustLive"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//         TVChannel {
//             name: String::from("NBC"),
//             channel_type: ChannelType::News,
//         },
//         TVChannel {
//             name: String::from("RustTV"),
//             channel_type: ChannelType::ProgrammingTutorials,
//         },
//     ];

//     let all_are_rust = channels
//         .iter()
//         .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

//     let any_are_rust = channels
//         .iter()
//         .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

//     println!("{all_are_rust}");
//     println!("{any_are_rust}");

//     // let good_channels: Vec<String> = channels
//     //     .iter()
//     //     .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
//     //     .map(|channel| channel.name.clone())
//     //     .collect();

//     // println!("{:#?}", good_channels.len() == channels.len());

//     // let good_channel = channels
//     //     .iter()
//     //     .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
//     // println!("{}", good_channel.is_some());
// }

// --------------------------------------------------------- //

// fn main() {
//     let teas = [
//         String::from("Hot Earl Grey"),
//         String::from("Iced Green"),
//         String::from("Hot Matcha"),
//     ];

//     let more_teas: Vec<String> = teas
//         .iter()
//         .filter(|tea| tea.contains("Hot"))
//         .cloned()
//         .collect();
//     println!("{more_teas:#?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let stocks = ["nvda", "", "aapl", "", "msft", "goog"];

//     let capitalized_stocks: Vec<String> = stocks
//         .iter()
//         .filter(|stock| !stock.is_empty())
//         .map(|stock| stock.to_uppercase())
//         .collect();
//     println!("{:?}", capitalized_stocks);

//     let capitalized_stocks: Vec<String> = stocks
//         .iter()
//         .filter_map(|stock| {
//             if stock.is_empty() {
//                 None
//             } else {
//                 Some(stock.to_uppercase())
//             }
//         })
//         .collect();
//     println!("{:?}", capitalized_stocks);
// }

// --------------------------------------------------------- //

// fn main() {
//     let spreadsheet = vec![[100, 200, 300], [123, 456, 789], [987, 654, 321]];

//     let values: Vec<i32> = spreadsheet.into_iter().flatten().collect();
//     println!("{:?}", values);
// }

// --------------------------------------------------------- //

// fn main() {
//     let attendees = [
//         "Bob, Mary, Kevin",
//         "Mike, Robbie, Matt, Austin",
//         "Piers, Liam",
//     ];

//     let attendees: Vec<&str> = attendees
//         .iter()
//         .flat_map(|group| group.split(", "))
//         .collect();

//     println!("{attendees:#?}");

// }

// --------------------------------------------------------- //

// fn main() {
//     let applicants = vec!["Bob", "Rob", "Cob", "Alex", "Piers", "John", "Dan"];

//     let winners: Vec<&str> = applicants
//         .into_iter()
//         .enumerate()
//         .filter_map(|(index, applicant)| {
//             if index % 3 == 0 {
//                 Some(applicant)
//             } else {
//                 None
//             }
//         })
//         .collect();

//     println!("{winners:?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let numbers = [4, 8, 15, 16, 23, 42];

//     let (evens, odds): (Vec<i32>, Vec<i32>) =
//         numbers.into_iter().partition(|number| number % 2 == 0);

//     // println!("{groups:?}");
//     println!("{evens:?}");
//     println!("{odds:?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let first_names = ["Casey", "Robert", "Cargo", "Dan"];
//     let last_names = ["Johnson", "Smith", "Rustman"];

//     for (first_name, last_name) in first_names.iter().zip(last_names) {
//         println!("{first_name} {last_name}");
//     }

//     let complete_names = first_names
//         .iter()
//         .zip(last_names)
//         .map(|(first_name, last_name)| format!("{first_name} {last_name}"))
//         .collect::<Vec<String>>();
//     println!("{complete_names:#?}");
// }

// --------------------------------------------------------- //

// use std::collections::HashMap;

// struct SupportStaff {
//     day: String,
//     employee: String,
// }

// fn main() {
//     let earnings = [4, 7, 9, 13];

//     let sum = earnings.into_iter().fold(0, |total, current| {
//         println!("Toatal: {total}, current: {current}");
//         total + current
//     });
//     println!("{sum}");

//     let week = [
//         SupportStaff {
//             day: String::from("Monday"),
//             employee: String::from("Brian"),
//         },
//         SupportStaff {
//             day: String::from("Tuesday"),
//             employee: String::from("Cam"),
//         },
//         SupportStaff {
//             day: String::from("Wednesday"),
//             employee: String::from("Walter"),
//         },
//     ];

//     let map = week
//         .into_iter()
//         .fold(HashMap::new(), |mut data, entry: SupportStaff| {
//             data.insert(entry.day, entry.employee);
//             data
//         });
//     println!("{map:?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let earnings = [4, 7, 9, 13];

//     let sum = earnings
//         .into_iter()
//         .reduce(|total, current| total + current);

//     println!("{sum:?}");

//     let address_portions = [
//         String::from("123 Elm Street"),
//         String::from("Suburbia"),
//         String::from("New Jersey"),
//     ];
//     println!("{}", address_portions.join(", "));

//     let address = address_portions
//         .into_iter()
//         .reduce(|mut accumulator, portion| {
//             accumulator.push_str(", ");
//             accumulator.push_str(&portion);
//             accumulator
//         });
//     println!("{address:?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     let total: i32 = numbers.iter().sum();
//     println!("{total}");

//     let product: i32 = numbers.iter().product();
//     println!("{product}");

//     let max = numbers.iter().max().unwrap();
//     println!("{max}");

//     let min = numbers.iter().min().unwrap();
//     println!("{min}");

//     let count = numbers.iter().count();
//     println!("{count}");

//     let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];

//     let sum = numbers
//         .iter()
//         .filter(|number| !number.is_nan())
//         .copied()
//         .fold(0.0, |total, current| total + current);
//     println!("{sum}");

//     let max = numbers
//         .iter()
//         .filter(|number| !number.is_nan())
//         .copied()
//         .reduce(|accum, current| accum.max(current));
//     println!("{max:?}");
// }

// --------------------------------------------------------- //

// fn main() {
//     let performers = ["Rustful Five", "Rust in Peace", "Rustin Bieber"];

//     let last = performers.iter().last().unwrap();
//     println!("{last}");

//     let second = performers.iter().nth(1).unwrap();
//     println!("{second}");

//     let last = performers.iter().nth_back(0).unwrap();
//     println!("{last}");

//     let second_to_last = performers.iter().nth_back(1).unwrap();
//     println!("{second_to_last}");

//     let target_index = performers
//         .iter()
//         .position(|element| *element == "Rustin Bieber");
//     println!("{:?}", target_index);
// }

// --------------------------------------------------------- //

// fn main() {
//     let fifty_numbers = 1..=50;

//     for number in fifty_numbers.clone().take(15).skip(5).step_by(2) {
//         println!("{number}/");
//     }

//     println!("{fifty_numbers:?}");
// }

// --------------------------------------------------------- //

// #[derive(Debug)]
// struct GasStation {
//     snack_count: u32,
//     manager: String,
//     employee_count: u32,
// }

// fn main() {
//     let mobil = GasStation {
//         snack_count: 100,
//         manager: String::from("Meg Mobil"),
//         employee_count: 3,
//     };

//     let exxon = GasStation {
//         snack_count: 130,
//         manager: String::from("Eric Exxon"),
//         employee_count: 4,
//     };

//     let shell = GasStation {
//         snack_count: 50,
//         manager: String::from("Shane Shell"),
//         employee_count: 2,
//     };

//     let mut stops = [mobil, exxon, shell];

//     stops.sort_by_key(|station| station.snack_count);
//     println!("{stops:#?}");

//     stops.sort_by_key(|station| -(station.employee_count as i32));
//     println!("{stops:#?}");
// }

// --------------------------------------------------------- //

// use std::fs;
// use std::io;

// fn main() -> Result<(), io::Error> {
//     let contents = fs::read_to_string("story.txt")?;

//     for line in contents.lines() {
//         println!("{line}");
//     }

//     Ok(())
// }

// --------------------------------------------------------- //

// use std::env;

// fn main() {
//     let args = env::args();

//     for arg in args {
//         println!("{arg}");
//     }
// }

// --------------------------------------------------------- //

// use std::env;
// use std::process;

// #[derive(Debug)]
// struct Settings {
//     video_file: String,
//     subtitles: bool,
//     high_definition: bool,
// }

// fn main() {
//     let settings = collect_settings();
//     println!("{:?}", settings);
// }

// fn collect_settings() -> Settings {
//     // target/debug/iterators -> rust.mp4 true false nonsense
//     let mut args = env::args().skip(1).take(3);

//     let video_file = args.next().unwrap_or_else(|| {
//         eprint!("No video file specified!");
//         process::exit(1);
//     });

//     let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));

//     let subtitles = settings.next().unwrap_or(false);
//     let high_definition = settings.next().unwrap_or(false);

//     Settings {
//         video_file,
//         subtitles,
//         high_definition,
//     }
// }

// --------------------------------------------------------- //

// use std::fs;
// use std::io;

// fn main() -> io::Result<()> {
//     for entry_result in fs::read_dir("./")? {
//         let entry = entry_result?;
//         let entry_name = entry.path();
//         let metadata = fs::metadata(&entry_name)?;
//         if metadata.is_file() {
//             println!("{entry_name:?}\n-------------");
//             let contents = fs::read_to_string(&entry_name)?;
//             println!("{contents}");
//         }
//     }

//     Ok(())
// }

// --------------------------------------------------------- //

// use std::collections::HashSet;

// #[derive(Debug)]
// struct Playlist {
//     songs: Vec<String>,
//     users: HashSet<String>,
// }

// impl FromIterator<(String, String)> for Playlist {
//     fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
//         let mut songs = Vec::new();
//         let mut users = HashSet::new();
//         for (song, user) in iter {
//             songs.push(song);
//             users.insert(user);
//         }
//         Self { songs, users }
//     }
// }
// fn main() {
//     let fifty_numbers = 1..=50;

//     let results = Vec::from_iter(fifty_numbers.clone());
//     println!("{results:?}");

//     let results: Vec<_> = fifty_numbers.clone().collect();
//     println!("{results:?}");

//     let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers);
//     println!("{unique_set:?}");

//     let songs = [
//         (String::from("I Rust Go On"), String::from("Bob")),
//         (String::from("A Rust of Wind"), String::from("Bob")),
//         (String::from("A Rustworthy Man"), String::from("Sheila")),
//     ];

//     // let playlist: Playlist = Playlist::from_iter(songs);
//     // println!("{playlist:?}");

//     let playlist = songs.into_iter().collect::<Playlist>();
// }

// --------------------------------------------------------- //

// use colored::Colorize;
// use std::io::{self, Write};

// fn main() {
//     let word = "trait";
//     let input = io::stdin();

//     for _ in 1..=6 {
//         let mut user_input = String::new();

//         input
//             .read_line(&mut user_input)
//             .expect("Failed to provide input");

//         for (word_character, user_character) in word.chars().zip(user_input.chars().take(5)) {
//             if word_character == user_character {
//                 print!("{}|", format!(" {user_character} ").on_green());
//             } else if word.contains(user_character) {
//                 print!("{}|", format!(" {user_character} ").on_yellow());
//             } else {
//                 print!("{}|", format!(" {user_character} ").on_black());
//             }

//             io::stdout().flush().unwrap();
//         }

//         println!();

//         if word == user_input.trim() {
//             println!("You got it! The word is {word}");
//             break;
//         }
//     }
// }
