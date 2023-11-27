//pub mod group_debts;
//pub mod menus;

pub mod group_debts_menu;

use crate::interface::{clear_terminal, get_selectable_option_from_list, print_message};
use crate::user::User;

use crate::menu::group_debts_menu::{group_debt_menu, new_group_debt_menu};

pub fn main_menu(user: &mut User) {
  clear_terminal();

  // if he has not groups
  if user.group_debts.len() == 0 {
    print_message("You don't have group debts yet, create one!");
    let options = vec!["Add new group"];
    let selected_option = get_selectable_option_from_list(&options).unwrap();

    if selected_option == 0 {
      new_group_debt_menu(&mut user.group_debts);
    }
  }

  clear_terminal();
  //println!("{user:?}");
  print_message("Your group debts:");

  let mut create_group_debts_options = user
    .group_debts
    .iter()
    .map(|group_debt| group_debt.name.as_str())
    .collect::<Vec<&str>>();

  create_group_debts_options.push("Add a new group debt");

  let new_group_debt_option_index = create_group_debts_options.len() - 1;

  let selected_option = get_selectable_option_from_list(&create_group_debts_options).unwrap();

  // if new group debt option is selected
  if new_group_debt_option_index == selected_option {
    new_group_debt_menu(&mut user.group_debts);
  }

  group_debt_menu(&mut user.group_debts[selected_option]);

  //new_group_debt_menus(&mut user.group_debts)
}
