use std::collections::HashMap;
use std::collections::HashSet;

// ----- Create a HashMap with new Function -----
// fn main() {
//     let mut menu: HashMap<String, f64> = HashMap::new();

//     menu.insert(String::from("Steak"), 29.99);
//     menu.insert(String::from("Tuna"), 29.99);
//     menu.insert(String::from("Burger"), 14.99);

//     println!("{menu:?}");

//     let mut country_capitals: HashMap<&str, &str> = HashMap::new();
//     country_capitals.insert("France", "Paris");
//     country_capitals.insert("Germany", "Berlin");

//     println!("{country_capitals:?}");
// }

// ----- The remove Method -----
// fn main() {
//     let data = [("Bobby", 7), ("Grant", 4), ("Ben", 6)];

//     let mut years_at_company = HashMap::from(data);
//     println!("{:?}", years_at_company);

//     let ben = years_at_company.remove("Ben");
//     println!("{:?}", ben);
//     println!("{:?}", ben.unwrap());
//     println!("{:?}", years_at_company);

//     let ben = years_at_company.remove("Ben");
//     println!("{:?}", ben);
// }

// ----- Hash Maps and Ownership -----
// fn main() {
//     let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairings.insert(&drink, &milk);
//     coffee_pairings.insert("Flat White", "Almond Milk");

//     println!("{:?}", coffee_pairings);
//     println!("{}", coffee_pairings.len());
//     println!("{drink} {milk}");
// }

// ----- Access a Value by Key -----
// fn main() {
//     let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairings.insert(&drink, &milk);
//     coffee_pairings.insert("Flat White", "Almond Milk");

//     // let value = coffee_pairings["Flat White"];
//     let value = coffee_pairings
//         .get("Flat White")
//         .copied()
//         .unwrap_or("Unknown Milk");
//     println!("{:?}", value)
// }

// ----- Overwriting a Value with an Existing Key -----
// fn main() {
//     let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairings.insert(&drink, &milk);
//     coffee_pairings.insert("Flat White", "Almond Milk");

//     println!("{:?}", coffee_pairings);
//     coffee_pairings.insert("Latte", "Pistachio Milk");
//     println!("{:?}", coffee_pairings);
// }

// ----- The entry Method -----
// fn main() {
//     let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();
//     let drink = String::from("Latte");
//     let milk = String::from("Oat Milk");
//     coffee_pairings.insert(&drink, &milk);
//     coffee_pairings.insert("Flat White", "Almond Milk");

//     coffee_pairings.entry("Latte").or_insert("Pistachio Milk");
//     println!("{:?}", coffee_pairings);

//     coffee_pairings
//         .entry("Cappuccino")
//         .or_insert("Pistachio Milk");
//     println!("{:?}", coffee_pairings);
// }

// ----- The HashSet -----
// fn main() {
//     let mut concert_queue: HashSet<&str> = HashSet::new();
//     println!("{:?}", concert_queue);

//     concert_queue.insert("Molly");
//     concert_queue.insert("Megan");
//     println!("{:?}", concert_queue);
//     println!("{}", concert_queue.len());

//     concert_queue.insert("Molly");
//     println!("{:?}", concert_queue);

//     println!("{}", concert_queue.remove("Megan"));
//     println!("{}", concert_queue.remove("Franny"));
//     println!("{:?}", concert_queue);

//     println!("{}", concert_queue.contains("Molly"));
//     println!("{}", concert_queue.contains("Fred"));

//     println!("{:?}", concert_queue.get("Molly"));
//     println!("{:?}", concert_queue.get("Joe"));
// }

// ----- HashSet Operations -----
fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new(); // Boris, Melissa
    let mut movie_queue: HashSet<&str> = HashSet::new(); // Boris, Phil

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", movie_queue.union(&concert_queue));

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    let mut attendees = HashSet::new();
    attendees.insert("Boris");
    println!("{}", attendees.is_subset(&concert_queue));

    println!("{}", concert_queue.is_superset(&attendees));
}
