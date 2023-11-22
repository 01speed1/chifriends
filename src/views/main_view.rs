use crate::interface::{
  clear_terminal,
  get_input,
  print_jump_line,
  // get_option_menu_number,
  print_message,
  // print_options,
  print_title,
  wait_for_press_enter,
};

use crate::users::User;

pub fn welcome_message_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;
  print_title("Welcome to Chifriends");
  print_jump_line();
  print_message("With this app you can share grupal debts with your friends ");
  print_jump_line();
  print_message("created by 01speed1");
  print_jump_line();
  print_message("Press enter to continue...");

  wait_for_press_enter()?;

  main_menu_view()?;

  Ok(())
}

fn new_user_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;
  print_title("New User");
  print_jump_line();
  print_message("You are a new user, we need a usernanme to create an account");
  print_jump_line();
  print_message("Enter your username:");

  let username = get_input();

  User::new(username);

  Ok(())
}

fn old_user_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;

  let mut title = User::get_username();
  title = format!("Hi again {}!", title);

  print_message(&title);

  

  Ok(())
}

pub fn main_menu_view() -> Result<(), Box<dyn std::error::Error>> {
  if !User::already_exists() {
    new_user_view()?;
  }

  old_user_view()?;

  Ok(())
}
