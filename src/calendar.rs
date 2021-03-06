use ansi_term::Colour;
use chrono::TimeZone;
use chrono::Weekday;
use chrono::{Datelike, Local};
use num_traits::cast::FromPrimitive;

use crate::date;

pub fn render_calendar(month: u32, year: i32) -> String {
  let today = Local::today();

  let mut result = String::from("");

  result.push_str(&format!(
    "{: <13} {}, {}\r\n",
    "",
    month_to_text(month),
    year
  ));
  for day in 0..7 {
    let weekday = Weekday::from_i64(day as i64).expect("failed to convert to week day");
    result.push_str(&format!("  {}", weekday));
  }
  result.push_str("\r\n");
  let calendar = fill_calendar(month, year);
  for day in calendar {
    match day.date {
      None => result.push_str("     "),
      Some(d) => {
        if today.day() == d && today.month() == month && today.year() == year {
          // TODO: There must be a better way to print this
          result.push_str(&format!(
            "{space: <width$}{date}",
            space = "",
            width = 5 - num_of_char(d),
            date = Colour::Black
              .bold()
              .on(Colour::White)
              .paint(format!("{}", d))
          ))
        } else {
          result.push_str(&format!("{:5}", d))
        }
      }
    }
    if day.weekday == Weekday::Sun {
      result.push_str("\r\n");
    }
  }
  result.push_str("\r\n");
  result
}

fn fill_calendar(month: u32, year: i32) -> Vec<Day> {
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

  vec![pre_month, days]
    .into_iter()
    .flatten()
    .collect()
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
