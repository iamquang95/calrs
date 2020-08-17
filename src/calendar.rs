use ansi_term::{Style, Colour};
use chrono::TimeZone;
use chrono::Weekday;
use chrono::{Datelike, Local};
use num_traits::cast::FromPrimitive;

use crate::date;

pub fn render_calendar(month: u32, year: i32) {
  let today = Local::today();
  let first_day = chrono::Local.ymd(year, month, 1);
  let native_today = first_day.naive_local();
  let days_of_month = date::get_days_of_month(native_today);

  let days: Vec<Day> = (1..=days_of_month)
    .map(|day| {
      let day = native_today
        .with_day(day as u32)
        .expect("failed to get day");
      Day {
        date: Some(day.day()),
        weekday: day.weekday(),
      }
    })
    .collect();

  let pre_month: Vec<Day> = (0..days[0].weekday.num_days_from_monday())
    .map(|day| {
      let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
      Day {
        date: None,
        weekday: weekday,
      }
    })
    .collect();

  let post_month: Vec<Day> = (0..days[0].weekday.num_days_from_monday())
    .map(|day| {
      let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
      Day {
        date: None,
        weekday: weekday,
      }
    })
    .collect();

  let calendar: Vec<Day> = vec![pre_month, days, post_month]
    .into_iter()
    .flatten()
    .collect();

  println!("{: <13} {}, {}", "", month_to_text(month), year);
  for day in 0..7 {
    let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
    print!("  {}", weekday);
  }
  println!();
  for day in calendar {
    match day.date {
      None => print!("     "),
      Some(d) => {
        if today.day() == d && today.month() == month && today.year() == year {
          print!("{space: <width$}{date}", space="", width=5-num_of_char(d), date=Colour::Black.bold().on(Colour::White).paint(format!("{}", d)))
        } else {
          print!("{:5}", d)
        }
      }
    }
    if day.weekday == Weekday::Sun {
      println!();
    }
  }
  println!();
}

fn num_of_char(n: u32) -> usize {
  let mut cnt = 0;
  let mut cur = n;
  loop {
    cur /= 10;
    cnt += 1;
    if cur == 0 {
      break;
    }
  }
  cnt
}

fn month_to_text(month: u32) -> String {
  String::from(match month {
    1 => "January",
    2 => "February",
    3 => "March",
    4 => "April",
    5 => "May",
    6 => "June",
    7 => "July",
    8 => "August",
    9 => "September",
    10 => "October",
    11 => "November",
    12 => "December",
    _ => "",
  })
}

struct Day {
  date: Option<u32>,
  weekday: Weekday,
}
