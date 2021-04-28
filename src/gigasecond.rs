use std::ops::Add;
use std::time::Duration;
use std::time::SystemTime;

pub fn after_gigasecond(start: SystemTime) -> SystemTime{
    let duration = Duration::new(1000000000, 0);
    return start.add(duration);
}
