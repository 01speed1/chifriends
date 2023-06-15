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

mod existing_user {
  use super::*;

  mod group_views {
    use super::*;

    pub fn new_group_view() -> Result<(), Box<dyn std::error::Error>> {
      clear_terminal()?;
      print_title("New Group");
      print_jump_line();
      print_message("You are creating a new group");
      print_jump_line();
      print_message("Enter the name of the group:");

      let group_name = get_input();

      User::add_group(&group_name);

      Ok(())
    }
  }

  pub fn user_view() -> Result<(), Box<dyn std::error::Error>> {
    clear_terminal()?;

    let mut title = User::get_username();
    title = format!("Hi again {}!", title);

    print_title(&title);

    Ok(())
  }
}

pub fn main_menu_view() -> Result<(), Box<dyn std::error::Error>> {
  if !User::already_exists() {
    new_user_view()?;
  }

  existing_user::user_view()?;

  Ok(())
}
