// ----- The panic! Macro -----
// fn main() {
//     let num = 2;
//     panic!("Something went wrong ");
// }

// ----- The process Module and the exit Function -----
// use std::process;

// fn main() {
//     // process::exit(0);

//     process::exit(1);
//     println!("This will not print");
// }

// ----- Standard Error (eprintln! Macro) -----
// fn main() {
//     println!("Some status update");
//     eprintln!("Error: Some error message");
// }

// ----- Opening a File -----
// use std::fs::File;
// use std::process;

// fn main() {
//     let file = match File::open("nonsense.txt") {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("Something went wrong reading the file. The error was {error:?}");
//             process::exit(1)
//         }
//     };

//     println!("{file:#?}");
// }

// ----- Asking the User for Input -----
// use std::fs::File;
// use std::io::stdin;
// use std::process;

// fn main() {
//     println!("Please enter the name of the file you'd like to read:");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         eprintln!("Something went wrong collecting user input. The error was {error:?}");
//         process::exit(1)
//     }

//     // .trim returns a string slice automatically
//     let file = match File::open(input.trim()) {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("Something went wrong reading the file. The error was {error}");
//             process::exit(1)
//         }
//     };

//     println!("{file:#?}");
// }

// ----- Reading the File's Contents -----
// use std::fs::File;
// use std::io::{Read, stdin};
// use std::process;

// fn main() {
//     println!("Please enter the name of the file you'd like to read:");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Err(error) = user_requested_file {
//         eprintln!("Something went wrong collecting user input. The error was {error:?}");
//         process::exit(1)
//     }

//     // .trim returns a string slice automatically
//     let mut file = match File::open(input.trim()) {
//         Ok(file) => file,
//         Err(error) => {
//             eprintln!("Something went wrong opening the file. The error was {error}");
//             process::exit(1)
//         }
//     };

//     let mut file_contents = String::new();
//     let read_operation = file.read_to_string(&mut file_contents);

//     if let Err(error) = read_operation {
//         eprintln!("Something went wrong reading the file as a string. The error was {error}");
//         process::exit(1)
//     }

//     println!("{file:#?}");
//     println!("{file_contents}");
// }

// ----- Propagating Errors-----
use std::fs::File;
use std::io::{self, Read, stdin};
use std::process;

fn main() {
    let file_result = read_file();

    match file_result {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            eprintln!("There was an errror: {error:?}")
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the name of the file you'd like to read:");
    let mut input = String::new();

    let user_requested_file = stdin().read_line(&mut input);

    if let Err(error) = user_requested_file {
        return Err(error);
    }

    // .trim returns a string slice automatically
    let mut file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };

    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    if let Err(error) = read_operation {
        return Err(error);
    }

    Ok(file_contents)
}
