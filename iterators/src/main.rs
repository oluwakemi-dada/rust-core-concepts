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

fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];

    for number in &my_vector {
        println!("{number}");
    }

    println!("{:?}", my_vector);

    let cities = vec![String::from("Phoenix"), String::from("Dallas")];

    // cities.iter()
    for city in &cities {
        println!("{}", city)
    }

    println!("{:?}", cities);
}
