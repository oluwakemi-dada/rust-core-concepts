// use rand::random;

// fn main() {
//     let random_float: f64 = random();
//     println!("{}", random_float * 100.0);

//     let random_int = random::<u8>();
//     println!("{}", random_int);
// }

// ---------------------------------------------------- //

// use rand::{rng, Rng};

// fn main() {
//   let mut my_rng = rng();
//   let random_float = my_rng.random::<f64>();
//   println!("{}", random_float);
  
//   let ten_random_values = (0..10).map(|_| my_rng.random::<i8>()).collect::<Vec<i8>>();
//   println!("{:?}", ten_random_values);
  
//   let random_number: i32 = my_rng.random_range(29..53);
//   println!("{random_number}",);
  
//   println!("{}", my_rng.random_bool(0.9));
  
//   println!("{}", my_rng.random_ratio(1, 2));
//   println!("{}", my_rng.random_ratio(9, 13));
// }

// ---------------------------------------------------- //

use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    let mut my_rng = rng();
    let mut candies = vec![
        "Sour Patch Kids",
        "Kit Kat",
        "Twix",
        "Snickers",
        "Starburst",
    ];
    candies.shuffle(&mut my_rng);

    println!("{:?}", candies);
}
