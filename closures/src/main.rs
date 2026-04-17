// fn main() {
//     let multiplier = 5;

//     // fn multiply_by(value: i32) -> i32 {
//     //     value * multiplier
//     // }

//     let multiply_by = |value: i32| -> i32 { return multiplier * value };

//     println!("{}", multiply_by(2));

//     let product = |a: i32, b: i32| -> i32 {
//         println!("Calculating product for you");
//         a * b
//     };

//     println!("{}", product(3, 9));
//     println!("{}", product(5, 8));
// }

// --------------------------------------------------- //

// fn main() {
//     let multiplier = 5;

//     let multiply_by = |value| value * multiplier;
//     println!("{}", multiply_by(3 as u8));

//     let numbers = vec![4, 8, 15, 16, 23, 42];

//     let print_number = || println!("{:?}", numbers);
//     print_number();
//     print_number();
//     print_number();
//     println!("{:?}", numbers);
// }

// --------------------------------------------------- //

// fn main() {
//     let mut numbers = vec![4, 8, 15, 16, 23, 42];
//     let mut add_number = || numbers.push(100);
//     add_number();
//     // println!("{:?}", numbers); // not possible
//     add_number();
//     add_number();
//     println!("{:?}", numbers);
// }

// --------------------------------------------------- //

fn main() {
    let number = 13;
    let capture_number = || number;

    let a = capture_number();
    let b = capture_number();
    println!("{a} {b} {number}");

    let first_name = String::from("Alice");
    let capture_string = || {
        let person = first_name;
        println!("{person}");
    };
    capture_string();
    // capture_string(); cannot be called more than once
  
}
