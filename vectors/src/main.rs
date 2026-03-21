// ---- Coding Challenge ----
#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let file = File { name };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new(String::from("Rust Core Concepts"));

    folder.create_file(String::from("ownership.rs"));
    folder.create_file(String::from("structs.rs"));
    folder.create_file(String::from("functions.rs"));
    println!("{folder:#?}");

    folder.delete_file(1);
    println!("{folder:#?}");

    // let file = folder.get_file(1);
    let file = folder.get_file(5);
    match file {
        Some(f) => print!("Retrieved file: {f:#?}"),
        None => println!("There was no file"),
    }
}

// ---- Create a Vector ----
// fn main() {
//     let pizza_diameters = vec![8, 10, 12, 14];
//     println!("{pizza_diameters:?}");

//     let pastas: Vec<&str> = vec!["Lidia", "Carlota", "Angeles", "Marga"];
//     println!("{pastas:?}");
// }

// ---- Adding and Removing Elements ----
// fn main() {
//     let mut pizza_diameters = vec![8, 10, 12, 14];
//     pizza_diameters.push(16);
//     pizza_diameters.push(18);

//     pizza_diameters.insert(0, 4);

//     let last_pizza_diameter = pizza_diameters.pop();
//     println!("{last_pizza_diameter:?}");

//     let third_diameter_from_start = pizza_diameters.remove(2);
//     println!("{third_diameter_from_start:?}");

//     println!("{pizza_diameters:?}");
// }

// ---- Reading Vector Elements ----
// fn main() {
//     let pizza_diameters = vec![8, 10, 12, 14];

//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];

//     let pizza_slice = &pizza_toppings[1..3];
//     println!("{pizza_slice:?}");
// }

// ---- The get Method ----
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];

//     let option = pizza_toppings.get(5);

//     match option {
//         Some(topping) => println!("The topping is {topping}"),
//         None => println!("No value at that index position"),
//     }
// }

// ---- Ownership with Vectors ----
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let pizza_toppings = vec![pepperoni, mushroom, sausage];
//     let mut delicious_toppings = pizza_toppings;

//     let topping_reference = &delicious_toppings[1];
//     println!("{topping_reference:?}");

//     delicious_toppings.push(String::from("Olives"));
//     // println!("The topping is {topping_reference:?}"); // this will not work
// }

// ---- Writing Vector Elements ----
// fn main() {
//     let pepperoni = String::from("Pepperoni");
//     let mushroom = String::from("Mushroom");
//     let sausage = String::from("Sausage");
//     let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

//     pizza_toppings[1] = String::from("Olives");
//     println!("{pizza_toppings:#?}");

//     let target_topping = &mut pizza_toppings[2];
//     target_topping.push_str(" and Meatballs");
//     let another_topping = &pizza_toppings[1];
//     let another_one = &pizza_toppings[1];

//     println!("{pizza_toppings:#?}");
//     println!("{another_topping:#?}");
//     println!("{another_one:#?}");
// }

// ---- Vector Capacity Behind the Scenes ----
// fn main() {
//     let mut seasons: Vec<&str> = Vec::with_capacity(4);
//     println!(
//         "Length: {}, Capacity: {}",
//         seasons.len(),
//         seasons.capacity()
//     );

//     seasons.push("Summer");
//     seasons.push("Fall");
//     seasons.push("Winter");
//     seasons.push("Spring");
//     println!(
//         "Length: {}, Capacity: {}",
//         seasons.len(),
//         seasons.capacity()
//     );

//     seasons.push("Summer");
//       println!(
//         "Length: {}, Capacity: {}",
//         seasons.len(),
//         seasons.capacity()
//     );
// }
