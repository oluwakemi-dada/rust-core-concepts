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

fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();
    println!("{more_teas:#?}");
}
