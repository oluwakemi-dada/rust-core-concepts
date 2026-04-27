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

// #[derive(Debug)]
// enum LinkedListUsingBox<T> {
//     Empty,
//     Node {
//         value: T,
//         next: Box<LinkedListUsingBox<T>>,
//     },
// }

// #[derive(Debug)]
// enum LinkedListUsingReference<'a, T> {
//     Empty,
//     Node {
//         value: T,
//         next: &'a LinkedListUsingReference<'a, T>,
//     },
// }

// fn create_list() -> LinkedListUsingBox<i32> {
//     let second_node = LinkedListUsingBox::Node {
//         value: 2,
//         next: Box::new(LinkedListUsingBox::Empty),
//     };

//     let first_node = LinkedListUsingBox::Node {
//         value: 1,
//         next: Box::new(second_node),
//     };

//     first_node
// }

// fn main() {
//     let list = create_list();
//     println!("{list:#?}");
// }

// ---------------------------------------------------------- //

// #[derive(Debug)]
// enum FileSystemEntity {
//     File {
//         name: String,
//     },
//     Folder {
//         name: String,
//         content: Vec<FileSystemEntity>,
//     },
// }

// fn main() {
//     let rust_file = FileSystemEntity::File {
//         name: String::from("my_rust_code.rs"),
//     };
//     let python_file = FileSystemEntity::File {
//         name: String::from("my_python_code.py"),
//     };
//         let code_folder = FileSystemEntity::Folder {
//         name: String::from("Code Stuff"),
//         content: vec![rust_file, python_file],
//     };
//     let screenplay = FileSystemEntity::File {
//         name: String::from("My Screenplay.txt"),
//     };
//     let all_documents = FileSystemEntity::Folder {
//         name: String::from("Documents"),
//         content: vec![screenplay, code_folder],
//     };

//     println!("{all_documents:#?}");
// }

// ---------------------------------------------------------- //

// use std::cmp::Ordering;

// #[derive(Debug)]
// enum BinarySearchTree {
//     Empty,
//     Node {
//         value: i32,
//         left: Box<BinarySearchTree>,
//         right: Box<BinarySearchTree>,
//     },
// }

// impl BinarySearchTree {
//     fn new() -> Self {
//         BinarySearchTree::Empty
//     }

//     fn insert(&mut self, new_value: i32) {
//         match self {
//             BinarySearchTree::Empty => {
//                 *self = BinarySearchTree::Node {
//                     value: new_value,
//                     left: Box::new(BinarySearchTree::Empty),
//                     right: Box::new(BinarySearchTree::Empty),
//                 }
//             }
//             BinarySearchTree::Node { value, left, right } => match new_value.cmp(value) {
//                 Ordering::Equal => (),
//                 Ordering::Less => left.insert(new_value),
//                 Ordering::Greater => right.insert(new_value),
//             },
//         }
//     }

//     fn contains(&self, target: i32) -> bool {
//         match self {
//             BinarySearchTree::Empty => false,
//             BinarySearchTree::Node { value, left, right } => match target.cmp(value) {
//                 Ordering::Equal => true,
//                 Ordering::Less => left.contains(target),
//                 Ordering::Greater => right.contains(target),
//             },
//         }
//     }
// }

// fn main() {
//     let mut tree = BinarySearchTree::new(); // []
//     tree.insert(5);
//     tree.insert(2);
//     tree.insert(8);
//     tree.insert(4);
//     tree.insert(13);
//     println!("{tree:#?}");

//     println!("{}", tree.contains(13)); // true
//     println!("{}", tree.contains(4)); // true
//     println!("{}", tree.contains(21)); // false
// }

// ---------------------------------------------------------- //

// use std::ops::{Deref, DerefMut};

// struct CustomBox<T> {
//     data: T,
// }

// impl<T> CustomBox<T> {
//     fn new(data: T) -> Self {
//         Self { data }
//     }
// }

// impl<T> Deref for CustomBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.data
//     }
// }

// impl<T> DerefMut for CustomBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.data
//     }
// }

// fn main() {
//     let mut boxy: Box<f64> = Box::new(3.14);
//     *boxy = 6.28;
//     println!("{}", *boxy); // 25

//     let mut custom_boxy = CustomBox::new(4);
//     *custom_boxy = 25;
//     println!("{}", *custom_boxy);
// }

// ---------------------------------------------------------- //

// use std::ops::{Deref, DerefMut};

// struct CustomBox<T, U> {
//     data: T,
//     more_data: U,
// }

// impl<T, U> CustomBox<T, U> {
//     fn new(data: T, more_data: U) -> Self {
//         Self { data, more_data }
//     }
// }

// impl<T, U> Deref for CustomBox<T, U> {
//     type Target = U;

//     fn deref(&self) -> &Self::Target {
//         &self.more_data
//     }
// }

// impl<T, U> DerefMut for CustomBox<T, U> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.more_data
//     }
// }
// impl<T, U> Drop for CustomBox<T, U> {
//     fn drop(&mut self) {
//         println!("I'm removing the CustomBox from memory!");
//     }
// }

// fn main() {
//     let mut boxy: Box<f64> = Box::new(3.14);
//     *boxy = 6.28;
//     println!("{}", *boxy); // 25

//     let mut custom_boxy = CustomBox::new(3.14, "Hello");
//     *custom_boxy = "Goodbye";
//     println!("{}", *custom_boxy);
// }

// ---------------------------------------------------------- //

fn main() {
    let text = String::from("Hello");
    let my_box = Box::new(text);
    let value = &(*my_box)[..];
    output_text(value);
}

fn output_text(text: &str) {
    println!("{}", text);
}
