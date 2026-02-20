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
fn main() {
  println!("Dear Emily,\nHow have you been?");
  println!("\tOnce upon a time");
  println!("Juliet said \"I love you Romeo\"");
  let filepath = r"C:\My Documents\new\videos";
  println!("{filepath}")
}