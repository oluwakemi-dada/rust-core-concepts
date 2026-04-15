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

fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = select_first_two_elements(&cities);
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}



