// Mods
mod constants;
mod interface;

mod views;

// Imports entities
mod users;

// Imports
use users::User;

use views::main_view::main_menu_view;

fn main() {

  

  let _main_user = User::new("");

    = match User::already_exists() {
    true => User = users::User::new("main_user");,
    false => println!("User does not exist"),
  }


  main_menu_view().unwrap();
}
