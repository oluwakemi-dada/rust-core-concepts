// fn main() {
//     let mut sushi = String::from("Yellowtail");
//     let sushi_raw_pointer_1 = &raw const sushi;
//     let sushi_raw_pointer_2: *const String = &sushi;
//     let sushi_raw_mutable_pointer_1 = &raw mut sushi;
//     let sushi_raw_mutable_pointer_2 = &raw mut sushi;

//     drop(sushi);

//     unsafe {
//         println!("{}", *sushi_raw_pointer_1);
//     }
//     println!("4")
// }

// ---------------------------------------------------------- //

// fn main() {
//     let my_box = Box::new(100);

//     println!("{}", *my_box);
//     println!("{my_box}");
//     println!("{}", my_box);
//     println!("{:?}", *my_box);

//     let your_box = my_box;
//     println!("{your_box}");
// }

// ---------------------------------------------------------- //

// #[derive(Debug)]
// enum LinkedList<T> {
//     Empty,
//     Node { value: T, next: Box<LinkedList<T>> },
// }

// fn main() {
//     let list = LinkedList::Node {
//         value: 1.3,
//         next: Box::new(LinkedList::Node {
//             value: 2.6,
//             next: Box::new(LinkedList::Node {
//                 value: 3.9,
//                 next: Box::new(LinkedList::Empty),
//             }),
//         }),
//     };
//     println!("{:#?}", list);

//     let im_with_you = LinkedList::Node {
//         value: String::from("I'm With You"),
//         next: Box::new(LinkedList::Empty),
//     };
//     let sk8er_boi = LinkedList::Node {
//         value: String::from("Sk8er Boi"),
//         next: Box::new(im_with_you),
//     };
//     let complicated = LinkedList::Node {
//         value: String::from("Complicated"),
//         next: Box::new(sk8er_boi),
//     };

//     println!("{complicated:#?}");
// }

// ---------------------------------------------------------- //

#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedListUsingBox<T>>,
    },
}

#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        next: &'a LinkedListUsingReference<'a, T>,
    },
}

fn create_list() -> LinkedListUsingBox<i32> {
    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node),
    };

    first_node
}

fn main() {
    let list = create_list();
    println!("{list:#?}");
}
