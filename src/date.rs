use chrono::{Local, Date};

pub fn get_today() -> Date<Local> {
    Local::today()
}