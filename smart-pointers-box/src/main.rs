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
fn main() {
    let my_box = Box::new(100);

    println!("{}", *my_box);
    println!("{my_box}");
    println!("{}", my_box);
    println!("{:?}", *my_box);

    let your_box = my_box;
    println!("{your_box}");
}
