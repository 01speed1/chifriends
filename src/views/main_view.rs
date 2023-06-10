use crate::interface::{
  clear_terminal, get_input, get_option_menu_number, print_message, print_options, print_title,
};

use crate::users::User;

fn welcome_message_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;
  print_title("Welcome to Chifriends");

  print_message("Press any key to continue...");

  let any_input = get_input();

  if any_input != "" {
    main_menu_view()?;
  }

  Ok(())
}

fn new_user_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;
  print_title("New User");

  print_message("Enter your username:");

  let username = get_input();

  let new_user = User::new(&username);

  println!("User created: {:?}", new_user);

  Ok(())
}

fn old_user_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;

  Ok(())
}

pub fn main_menu_view() -> Result<(), Box<dyn std::error::Error>> {
  clear_terminal()?;
  print_title("Chifriends");

  Ok(())
}
