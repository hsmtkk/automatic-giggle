mod gigasecond;

use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

fn main() {

    let t = gigasecond::after_gigasecond(SystemTime::now());
    let dt: DateTime<Utc> = t.into();
    println!("{}", dt);
}
