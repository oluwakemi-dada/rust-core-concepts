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
// #![allow(unused_variables)]
// type Meters = i32;

// fn main() {
//   let mile_race_length: Meters = 1600;
//   let two_miles_race_length: Meters = 3200;
// }

// ---- Coding Challenge ----
const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
  let season = "Harmattan";
  let mut points_scored = 28;
  points_scored = 35;

  let event_time = "06:00";
  let event_time = 6;
  
  println!("My favorite season is {season}. The team scored {points_scored}. The event started at {event_time}. A touchdown is worth {TOUCHDOWN_POINTS} points");

  println!("My favorite season is {0}. The team scored {1}. The event started at {2}. A touchdown is worth {3} points", season, points_scored, event_time, TOUCHDOWN_POINTS);

  #[allow(unused_variables)]
  let favorite_beverage = "Water";
}