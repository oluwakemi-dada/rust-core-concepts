// ---- Variables ----
// fn main() {
//   let apples = 50;
//   let oranges = 14 + 6;
//   let _fruits = apples + oranges;

//   println!("This year, my garden has {0} apples and {1} oranges. I cannot believe i have {1} oranges.", apples, oranges);
// }

// ---- Immutable and Mutable Variables ----
// fn main() {
//   let mut gym_reps = 10;
//   println!("I plan to do {gym_reps} reps");

//   gym_reps = 15;
//     println!("I now plan to do {gym_reps} reps");
// }

// ---- Variable Shadowing ----
// fn main() {
//   let grams_of_protein = "100.345";
//   println!("{grams_of_protein}");
//   let grams_of_protein = 100.345;
//   println!("{grams_of_protein}");
//   let mut grams_of_protein = 100;
//   println!("{grams_of_protein}");
//   grams_of_protein = 105;
//   println!("{grams_of_protein}");
// }

// ---- Scopes ----
// fn main() {
//   let coffee_price = 5.99;

//   {
//     let coffee_price = 1.99;
//       println!("The coffee price is {coffee_price}")
//   }

//   println!("The coffee price is {coffee_price}")
// }

// ---- Constants ----
// const TAX_RATE: f64 = 7.25; 

// fn main() {
//   let income = 100000;
//   println!("My income is {income} and my tax rate is {TAX_RATE}")
// }

// ---- Type Aliases ----
// type Meters = i32;

// fn main() {
//   let mile_race_length: Meters = 1600;
//   let two_miles_race_length: Meters = 3200;
//   println!("A one miles race is {mile_race_length} meters long and a two miles race is {two_miles_race_length} meters long")
// }

// ---- Compiler Directives ----
#![allow(unused_variables)]
type Meters = i32;

fn main() {
  let mile_race_length: Meters = 1600;
  let two_miles_race_length: Meters = 3200;
}