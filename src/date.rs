use chrono::Datelike;
use chrono::{NaiveDate};

pub fn get_days_of_month(date: NaiveDate) -> i64 {
  let year = date.year();
  let month = date.month();
  let first_day_this_month = NaiveDate::from_ymd(year, month, 1);
  let first_day_next_month = NaiveDate::from_ymd(
    match month {
      12 => year + 1,
      _ => year,
    },
    match month {
      12 => 1,
      _ => month + 1,
    },
    1,
  );
  return first_day_next_month
    .signed_duration_since(first_day_this_month)
    .num_days();
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_get_days_of_month(
    year: i32,
    expected_feb_days: i32
  ) {
    for month in 1..=12 {
      match month {
        2 => {
          for day in 1..=expected_feb_days {
            let date_this_month = NaiveDate::from_ymd(year, month, day as u32);
            assert_eq!(get_days_of_month(date_this_month), expected_feb_days as i64)
          }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
          for day in 1..=31 {
            let date_this_month = NaiveDate::from_ymd(year, month, day);
            assert_eq!(get_days_of_month(date_this_month), 31)
          }
        }
        _ => {
          for day in 1..=30 {
            let date_this_month = NaiveDate::from_ymd(year, month, day);
            assert_eq!(get_days_of_month(date_this_month), 30)
          }
        }
      }
    }
  }

  #[test]
  fn get_days_of_month_non_leap_year() {
    test_get_days_of_month(2021, 28)
  }

  #[test]
  fn get_days_of_month_leap_year() {
    test_get_days_of_month(2020, 29)
  }
}
