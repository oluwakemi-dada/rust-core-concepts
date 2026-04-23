use chrono::NaiveDate;

fn main() {
    let birthday = "1997-11-29";
    let birthday = birthday
        .parse::<NaiveDate>()
        .expect("Unable to parse NaiveDate from string");
    println!("{birthday:?}");
}
