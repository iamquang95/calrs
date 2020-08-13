use chrono::{Local, Weekday};
use chrono::Datelike;
use num_traits::cast::FromPrimitive;

use crate::date;

pub fn render_calendar() {
  let today = Local::today();
  let native_today = today.naive_local();
  let days_of_month = date::get_days_of_month(native_today);

  let days: Vec<Day> = (1..=days_of_month).map(|day| {
    let day = native_today.with_day(day as u32).expect("failed to get day");
    Day {
      date: Some(day.day()),
      weekday: day.weekday(),
    }
  }).collect();

  let pre_month: Vec<Day> = (0..days[0].weekday.num_days_from_monday()).map(|day| {
    let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
    Day {
      date: None,
      weekday: weekday,
    }
  }).collect();

  let post_month: Vec<Day> = (0..days[0].weekday.num_days_from_monday()).map(|day| {
    let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
    Day {
      date: None,
      weekday: weekday,
    }
  }).collect();

  let calendar: Vec<Day> = vec![pre_month, days, post_month].into_iter().flatten().collect();
  
  for day in 0..7 {
    let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
    print!("  {}", weekday);
  }
  println!();
  
  for day in calendar {
    match day.date {
      None => print!("     "),
      Some(d) => print!("{:5}", d),
    }
    if day.weekday == Weekday::Sun {
      println!();
    }
  }
  println!();
}

struct Day {
  date: Option<u32>,
  weekday: Weekday
}