// use std::cell::Cell;

// #[derive(Debug)]
// struct ConcertTicket {
//     section: String,
//     seat: String,
//     scanned: Cell<bool>,
// }

// impl ConcertTicket {
//     fn new(section: String, seat: String) -> Self {
//         Self {
//             section,
//             seat,
//             scanned: Cell::new(false),
//         }
//     }

//     fn admit_attendee(&self) {
//         self.scanned.set(true);
//     }
// }

// fn main() {
//     let ticket = ConcertTicket::new(String::from("A"), String::from("3"));
//     println!("{}", ticket.scanned.get());

//     ticket.admit_attendee();
//     println!("{}", ticket.scanned.get());
// }

// ---------------------------------------------------- //

use std::cell::RefCell;

#[derive(Debug)]
struct ConcertTicket {
    section: String,
    seat: String,
    scanned: bool,
}

impl ConcertTicket {
    fn new(section: String, seat: String) -> Self {
        Self {
            section,
            seat,
            scanned: false,
        }
    }
}

fn main() {
    let ticket = RefCell::new(ConcertTicket::new(String::from("A"), String::from("3")));

    {
        let mut one = ticket.borrow_mut();
        one.seat = String::from("D");
    }

    // ticket.borrow_mut().seat = String::from("D");
    println!("{:#?}", ticket.borrow().seat);

    ticket.borrow_mut().seat = String::from("K");
    println!("{:#?}", ticket.borrow().seat);

    // println!("{:#?}", ticket);
}
