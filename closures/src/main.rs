fn main() {
    let multiplier = 5;

    // fn multiply_by(value: i32) -> i32 {
    //     value * multiplier
    // }

    let multiply_by = |value: i32| -> i32 { return multiplier * value };

    println!("{}", multiply_by(2));

    let product = |a: i32, b: i32| -> i32 {
        println!("Calculating product for you");
        a * b
    };

    println!("{}", product(3, 9));
    println!("{}", product(5, 8));
}
