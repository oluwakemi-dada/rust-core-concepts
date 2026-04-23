use rand::random;

fn main() {
    let random_float: f64 = random();
    println!("{}", random_float * 100.0);

    let random_int = random::<u8>();
    println!("{}", random_int);
}
