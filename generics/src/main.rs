// ---- Intro to Generics ----
#[derive(Debug)]
struct DeliSandwich {}
fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity::<u32>(5));
    println!("{}", identity::<f64>(13.5));
    println!("{}", identity::<&str>("Hello"));
    println!("{}", identity::<String>(String::from("Hi there!")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}
