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

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1
        }
    }

    counts
}

fn main() {
    println!(
        "{:#?}",
        count_words("Sally sells sea shells by the sea shore")
    );
}
