use chrono::{Date, Local, NaiveDate};
use chrono::Datelike;

use crate::date;

pub fn render_calendar() {
  let today = Local::today();
  let native_today = today.naive_local();
  let days_of_month = date::get_days_of_month(native_today);
  for d in 1..=days_of_month {
    let day = native_today.with_day(d as u32).expect("failed to get day");
    print!("({} - {}) ", d, day.weekday());
  }
}