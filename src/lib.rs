mod calendar;
mod date;

use chrono::{Datelike, Local};

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn render_current_month() {
  let today = Local::today();
  let month = today.month();
  let year = today.year();
  println!("{}", calendar::render_calendar(month, year));
}

pub fn run() {
  let today = Local::today();
  let mut month = today.month();
  let mut year = today.year();

  let stdin = stdin();
  let mut stdout = stdout().into_raw_mode().unwrap();

  write!(
    stdout,
    "{}{}{}{}",
    termion::clear::All,
    termion::cursor::Goto(1, 1),
    calendar::render_calendar(month, year),
    termion::cursor::Hide
  )
  .unwrap();
  stdout.flush().unwrap();

  for c in stdin.keys() {
    write!(
      stdout,
      "{}{}",
      termion::cursor::Goto(1, 1),
      termion::clear::CurrentLine
    )
    .unwrap();

    match c.unwrap() {
      Key::Char('q') => break,
      Key::Left => {
        let (new_month, new_year) = decrease_month_year(month, year);
        month = new_month;
        year = new_year;
      }
      Key::Right => {
        let (new_month, new_year) = increase_month_year(month, year);
        month = new_month;
        year = new_year;
      }
      Key::Backspace => {
        month = today.month();
        year = today.year();
      }
      _ => {}
    }
    write!(
      stdout,
      "{}{}",
      termion::clear::All,
      calendar::render_calendar(month, year)
    ).unwrap();
    stdout.flush().unwrap();
  }

  write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn increase_month_year(month: u32, year: i32) -> (u32, i32) {
  if month == 12 {
    (1, year + 1)
  } else {
    (month + 1, year)
  }
}

fn decrease_month_year(month: u32, year: i32) -> (u32, i32) {
  if month == 1 {
    (12, year - 1)
  } else {
    (month - 1, year)
  }
}
