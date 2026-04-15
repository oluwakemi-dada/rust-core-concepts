// fn main() {
//     let a = 1;

//     {
//         let b = 2;
//     }

//     let c = String::from("Winter");
//     // let d = c;
//     drop(c);
// }

// --------------------------------------------- //
fn main() {
    let dog = String::from("Watson");

    {
        let my_pet = &dog;
        println!("{my_pet}");
    }

    println!("{dog}");

    {
        let my_pet = &dog;
        println!("{my_pet}");
    }
}
