// use chrono::NaiveDate;

// fn main() {
//     let birthday = "1997-11-29";
//     let birthday = birthday
//         .parse::<NaiveDate>()
//         .expect("Unable to parse NaiveDate from string");
//     println!("{birthday:?}");
// }

// ------------------------------------------------------------------- //

// use chrono::TimeDelta;

// fn main() {
//     let five_seconds = TimeDelta::new(5, 0);
//     println!("{five_seconds:?}"); // Some(TimeDelta { secs: 5, nanos: 0 })

//     let _invalid = TimeDelta::new(5, 1_000_000_000);

//     let negative_five_seconds = TimeDelta::new(-5, 0).unwrap();
//     println!("{negative_five_seconds:?}");

//     let five_minutes = TimeDelta::minutes(5);
//     println!("{five_minutes:?}");

//     let negative_five_numbers = TimeDelta::minutes(-5);
//     println!("{negative_five_numbers:?}");

//     let five_hours = TimeDelta::hours(5);
//     println!("{five_hours:?}");

//     let five_days = TimeDelta::days(5);
//     println!("{five_days:?}");

//     let five_weeks = TimeDelta::weeks(5);
//     println!("{five_weeks:?}");

//     let total_duration = five_weeks + five_days + five_hours + five_minutes;

//     println!(
//         "{} weeks, {} days, {} hours, {} minutes",
//         total_duration.num_weeks(),
//         total_duration.num_days(),
//         total_duration.num_hours(),
//         total_duration.num_minutes()
//     );
// }

// ------------------------------------------------------------------- //

// use chrono::{NaiveDate, TimeDelta};
// use std::ops::{Add, Sub};

// fn main() {
//     let birthday = NaiveDate::from_ymd_opt(1997, 11, 29).unwrap();
//     println!("{}", birthday.add(TimeDelta::days(5)));

//     println!("{}", birthday.add(TimeDelta::weeks(2) + TimeDelta::days(5)));

//     println!("{}", birthday.sub(TimeDelta::weeks(3)));

//     println!("{}", birthday + TimeDelta::days(5));

//     println!("{}", birthday + (TimeDelta::weeks(2) + TimeDelta::days(5)));

//     println!("{}", birthday - TimeDelta::weeks(3));
//     println!("{}", birthday + TimeDelta::weeks(-3));

//     // println!("{}", birthday.add(TimeDelta::days(100_000_000)));
// }

// ------------------------------------------------------------------- //

// use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

// fn main() {
//     let four_thirty_am = NaiveTime::from_hms_opt(4, 30, 0);
//     println!("{:?}", four_thirty_am);

//     let four_thirty_pm = NaiveTime::from_hms_opt(16, 30, 0);
//     println!("{:?}", four_thirty_pm);

//     let day = NaiveDate::from_ymd_opt(1969, 7, 20).unwrap();
//     let time = NaiveTime::from_hms_opt(20, 17, 0).unwrap();
//     let moon_landing = NaiveDateTime::new(day, time);
//     println!("{moon_landing:?}");

//     println!("{}", moon_landing + TimeDelta::days(1000));
//     println!(
//         "{}",
//         moon_landing + TimeDelta::days(1000) + TimeDelta::minutes(45)
//     );
// }

// ------------------------------------------------------------------- //

// use chrono::prelude::*;
// use chrono_tz::Etc::GMTPlus1;

// fn main() {
//     let system_time = Local::now();
//     let utc_time = Utc::now();

//     println!("1) {}", system_time);
//     println!("2) {}", utc_time);

//     println!("3) {}", system_time.date_naive());
//     println!("4) {}", utc_time.date_naive());

//     println!("5) {}", system_time.time());
//     println!("6) {}", utc_time.time());

//     println!("7) {}", system_time.year());
//     println!("8) {}", utc_time.year());

//     println!("9) {}", system_time.month());
//     println!("10) {}", utc_time.month());

//     println!("11) {}", system_time.day());
//     println!("12) {}", utc_time.day());

//     println!("13) {}", system_time.hour());
//     println!("14) {}", utc_time.hour());

//     println!("15) {}", system_time.minute());
//     println!("16) {}", utc_time.minute());

//     println!("17) {}", system_time.second());
//     println!("18) {}", utc_time.second());

//     println!("19) {}", system_time.offset());
//     println!("20) {}", utc_time.offset());
// }

// ------------------------------------------------------------------- //

use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;

fn main() {
    let local_time = Local::now();
    let utc_time = local_time.with_timezone(&Utc);

    println!("{}", local_time);
    println!("{}", utc_time);

    println!("{}", utc_time.with_timezone(&Local));

    let la_time = local_time.with_timezone(&Los_Angeles);

    println!("{}", local_time);
    println!("{}", la_time);
}
