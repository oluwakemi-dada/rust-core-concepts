// Doesn't need lifetime annotation because it doesn't return a reference
fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

// Doesn't need lifetime annotation because it accepts one reference parameter (even though it returns a reference)
fn last_two<T>(collection: &[T]) -> &[T] {
    &collection[collection.len() - 2..]
}

// Needs lifetime annotation because it accepts more than one reference parameter and returns a reference
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{announcement}");
    &text[..5]
}

// Needs lifetime annotation because it accepts more than one reference parameter and returns a reference linked to either of the parameters
fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        first
    } else {
        second
    }
}

fn main() {
    println!("{}", double_the_length(&vec![1, 2, 3]));
    println!("{}", double_the_length(&vec![1, 2, 3, 4]));

    println!("{:?}", last_two(&vec![1, 2, 3]));
    println!("{:?}", last_two(&vec![1, 2, 3, 4, 5, 6]));

    println!("{}", first_five("refrigerator", "Hello"));

    println!(
        "{}",
        find_string_that_has_content("programming", "dining", "gram")
    )
}

// ---------------------------------------------------- //

// fn main() {
//     let a = 1;

//     {
//         let b = 2;
//     }

//     let c = String::from("Winter");
//     // let d = c;
//     drop(c);
// }

// ---------------------------------------------------- //

// fn main() {
//     let dog = String::from("Watson");

//     {
//         let my_pet = &dog;
//         println!("{my_pet}");
//     }

//     println!("{dog}");

//     {
//         let my_pet = &dog;
//         println!("{my_pet}");
//     }
// }

// ---------------------------------------------------- //

// fn main() {
//     let dog = String::from("Watson");
//     let my_pet = &dog;
//     println!("{my_pet};")
//     // 100 lines of code
// }

// ---------------------------------------------------- //

// fn main() {
//     // let some_cities = {
//     //     let cities = vec![
//     //         String::from("London"),
//     //         String::from("New York"),
//     //         String::from("Barcelona"),
//     //     ];
//     //     // Invalid line
//     //     &cities[..2]
//     // };

//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];

//     let favorite_cities = &cities[0..2];
//     println!("{favorite_cities:?}");
//     let places = cities;
// }

// ---------------------------------------------------- //

// fn select_first_two_elements(items: &[String]) -> &[String] {
//     &items[..2]
// }

// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = select_first_two_elements(&cities);
//     println!("{two_cities:?}");

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{two_coffees:?}");
//     }
// }

// ---------------------------------------------------- //

// fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
//     &items[..2]
// }

// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = {
//         let cities_ref = &cities;
//         select_first_two_elements(cities_ref)
//     };
//     println!("{two_cities:?}");

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{two_coffees:?}");
//     }
// }

// ---------------------------------------------------- //

// fn my_awesome_function(first: &i32, second: String) -> &i32 {
//     first
// }

// fn select_first_two_elements(items: &[String]) -> &[String] {
//     &items[0..2]
// }

// fn main() {
//     let cities = vec![
//         String::from("London"),
//         String::from("New York"),
//         String::from("Barcelona"),
//     ];
//     let two_cities = {
//         let cities_ref = &cities;
//         select_first_two_elements(cities_ref)
//     };
//     println!("{two_cities:?}");

//     {
//         let coffees = [String::from("Latte"), String::from("Mocha")];
//         let two_coffees = select_first_two_elements(&coffees);
//         println!("{two_coffees:?}");
//     }
// }

// ---------------------------------------------------- //

// fn choose_favorite<'a>(first: &str, second: &'a str) -> &'a str {
//     println!("{first}");
//     second
// }

// fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
//     println!("The second is {second}");
//     first
// }

// fn main() {
//     let orlando = String::from("Orlando");
//     let result = {
//         let san_francisco = String::from("San Francisco");
//         longest(&orlando, &san_francisco)
//     };
//     println!("{result}")
// }

// ---------------------------------------------------- //

// struct DentistAppointment {
//     doctor: String,
// }

// impl DentistAppointment {
//     fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
//         println!(
//             "You are booked from {} to {} with doctor {}",
//             check_in_time, check_out_time, self.doctor
//         );
//         check_in_time
//     }
// }

// fn main() {
//     let appt = DentistAppointment {
//         doctor: String::from("David"),
//     };
//     let result = appt.book("03:00PM", "11:00AM");
//     drop(appt);
//     println!("{result}");
// }

// ---------------------------------------------------- //

// #[derive(Debug)]
// struct TrainSystem<'a> {
//     name: &'a str,
// }

// fn main() {
//     let name = String::from("AmTrak");
//     let nj_transit = { TrainSystem { name: &name } };

//     println!("{:#?}", nj_transit.name);
// }

// ---------------------------------------------------- //

// #[derive(Debug)]
// struct TravelPlan<'a, 'b> {
//     from: &'b str,
//     to: &'a str,
// }

// fn main() {
//     let from = String::from("Portland");
//     let plan = figure_out_ending_point(&from);
//     println!("{plan}");
// }

// fn figure_out_ending_point(from: &str) -> &str {
//     let to = String::from("Bangor");

//     let travel_plan = TravelPlan {
//         from: &from,
//         to: &to,
//     };
//     travel_plan.from
// }

// ---------------------------------------------------- //

// const COUNT: i32 = 400;

// fn say_hello() -> &'static str {
//     "Hello"
// }

// fn value() -> &'static i32 {
//     &COUNT
// }

// fn main() {
//     let greeting = say_hello();
//     println!("{greeting}");

//     let value = value();
//     println!("{value}");
// }
