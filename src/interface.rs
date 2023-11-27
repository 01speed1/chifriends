//Notes: This file contains all the functions that are used to interact with the user

use crossterm::terminal::{Clear, ClearType};
use dialoguer::{theme::ColorfulTheme, Error as DialoguerError, Select};
use std::io::{self, Write};

pub fn clear_terminal() {
  let mut stdout = io::stdout();
  crossterm::queue!(
    stdout,
    Clear(ClearType::All),
    crossterm::cursor::MoveTo(0, 0)
  );

  stdout.flush();
}

pub fn get_option_menu_number() -> i32 {
  let mut buffer = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to get value");

  buffer.trim().parse::<i32>().unwrap()
}

pub fn get_input() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

pub fn print_message(message: &str) {
  println!("{}", message);
}

pub fn print_line() {
  println!("---------------------");
}

pub fn print_title(title: &str) {
  print_line();
  println!("{}", title);
  print_line();
}

pub fn print_options(options: &Vec<String>) {
  for (index, option) in options.iter().enumerate() {
    println!("{}: {}", index + 1, option);
  }
}

pub fn print_jump_line() {
  println!("\n");
}

pub fn get_selectable_option_from_list(options: &[&str]) -> Result<usize, DialoguerError> {
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
