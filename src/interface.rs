use std::io::{self, Write};

use crossterm::terminal::{Clear, ClearType};

pub fn clear_terminal() -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = io::stdout();
  crossterm::queue!(
    stdout,
    Clear(ClearType::All),
    crossterm::cursor::MoveTo(0, 0)
  )?;

  Ok(())
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

pub fn wait_for_press_enter() -> Result<bool, Box<dyn std::error::Error>> {
  println!("Press enter to continue...");
  let mut input = String::new();
  io::stdout().flush()?;
  io::stdin().read_line(&mut input)?;

  Ok(true)
}
