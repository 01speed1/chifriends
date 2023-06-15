// Mods
mod constants;
mod interface;

mod views;

// Imports entities
mod groups;
mod users;

// Imports
// use users::User;

use views::main_view::welcome_message_view;

fn main() {
  //loop {
  welcome_message_view().unwrap();
  //}
}
