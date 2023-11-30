mod interface;
mod menu;
mod friend;
mod group_debt;
mod user;

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
