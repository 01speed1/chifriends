mod friend;
mod group_debt;
mod interface;
mod menu;
mod user;

use crate::menu::main_menu;
use user::User;

fn main() {
  let mut user = User::load_from_file().unwrap();

  loop {
    main_menu(&mut user);
  }
}
