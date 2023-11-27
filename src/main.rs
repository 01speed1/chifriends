// Mods
mod constants;
mod interface;

mod menu;

// Imports entities
mod friend;
mod group_debt;
mod groups;
mod user;
//mod users;

// Imports
use crate::menu::main_menu;
use user::User;

fn main() {
  let mut main_user = User {
    group_debts: vec![],
  };
  loop {
    main_menu(&mut main_user);
  }
}
