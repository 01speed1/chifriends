//Notes: This file contains all the functions that are used to interact with the user

use crossterm::terminal::{Clear, ClearType};
use dialoguer::{theme::ColorfulTheme, Error as DialoguerError, Select};
use std::io::{stdin, stdout};

pub fn clear_terminal() {
  let mut stdout = stdout();
  crossterm::queue!(
    stdout,
    Clear(ClearType::All),
    crossterm::cursor::MoveTo(0, 0)
  )
  .unwrap();
}

pub fn get_input() -> String {
  let mut buffer = String::new();
  stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

pub fn print_message(message: &str) {
  println!("{}", message);
}

pub fn print_jump_line() {
  println!("\n");
}

pub fn get_selectable_option_from_list(options: &Vec<&str>) -> Result<usize, DialoguerError> {
  let options_with_index = options
    .iter()
    .enumerate()
    .map(|(index, option)| format!("{}. {}", index + 1, option))
    .collect::<Vec<String>>();

  Select::with_theme(&ColorfulTheme::default())
    .items(&options_with_index)
    .default(0)
    .interact()
}
