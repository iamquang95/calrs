use chrono::{Date, Local};

mod calendar;
mod date;

fn main() {
    println!("Hello, world!");
    calendar::render_calendar();
}
