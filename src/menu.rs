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

  user.save_to_file().unwrap();

  print_message("Your group debts:");

  let create_group_debts_options = user
    .group_debts
    .iter()
    .map(|group_debt| -> String {
      if group_debt.all_friends_paid() {
        format!("{} (✓)", group_debt.name)
      } else {
        group_debt.name.clone()
      }
    })
    .collect::<Vec<String>>();

  let mut options_as_ref: Vec<&str> = create_group_debts_options
    .iter()
    .map(AsRef::as_ref)
    .collect();

  options_as_ref.push("Add a new group debt");

  let new_group_debt_option_index = options_as_ref.len() - 1;

  let selected_option = get_selectable_option_from_list(&options_as_ref).unwrap();

  // if new group debt option is selected
  if new_group_debt_option_index == selected_option {
    new_group_debt_menu(&mut user.group_debts);
  }

  user.save_to_file().unwrap();

  group_debt_menu(&mut user.group_debts[selected_option]);
}
