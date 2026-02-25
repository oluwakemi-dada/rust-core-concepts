// ---- Integers ----
// fn main() {
//   let sixteen_bit_signed: i16 = -32500;
//   let sixteen_bit_unsigned: u16 = 64000;

//   let thirty_two_bit_signed: i32 = -2147483648;
//   let thirty_two_bit_unsigned: u32 = 4294967295;

//   let some_value = 20u16;
// }

// ---- Underscore "_" as visual separator
// fn main() {
//   let sixteen_bit_signed: i32 = 320_500;
// }

// ---- The usize and isize types ----
// fn main() {
//   let days: usize = 55;
//   let years: isize = -15_000;
// } 

//  ---- Strings and Raw Strings ----
// fn main() {
//   println!("Dear Emily,\nHow have you been?");
//   println!("\tOnce upon a time");
//   println!("Juliet said \"I love you Romeo\"");
//   let filepath = r"C:\My Documents\new\videos";
//   println!("{filepath}")
// }

//  ---- Intro to Methods ----
// fn main(){
//   let value: i32 = -15;
//   println!("{}", value.abs());

//   let empty_space = "    my content ";
//   println!("{}", empty_space.trim());

//   println!("{}", value.pow(2));
//   println!("{}", value.pow(3));
// }

//  ---- Floating Point Types ----
// fn main(){
//   let pi: f64 = 3.141593458972679344785973674;
//   println!("The current value of pi is {pi}");

//   println!("{}", pi.floor());
//   println!("{}", pi.ceil());
//   println!("{}", pi.round());
// }

//  ---- Formatting Floats with Format Specifier ----
// fn main(){
//   let pi: f64 = 3.141593458972679344785973674;
//   println!("The current value of pi is {0} and the formatted value of pi is {0:.3}", pi);
// }

//  ---- Casting Types with the as Keyword ----
// fn main(){
//   let miles_away = 50;
//   let miles_away_i8 = miles_away as i8;

//   let miles_away = 100.329032;
//   let miles_away_f32 = miles_away as f32;
//   let miles_away_int = miles_away as i32;

//   println!("{miles_away_int}")
// }

//  ---- Math Operations ----
fn main() {
  let addition = 5 + 4;
  let subtraction = 10 - 6;
  let multiplication = 3 * 4;
  println!("Addition: {addition}, subtraction: {subtraction}, multiplication: {multiplication}");

  let floor_division = 5 / 3;
  println!("{floor_division}");

  let decimal_division = 5.0 / 3.0;
  println!("{decimal_division}");

  let remainder = 7 % 2;
  println!("{remainder}");
}