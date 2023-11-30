use crate::friend::Friend;
use crate::group_debt::GroupDebt;
use crate::interface::{
  clear_terminal, get_input, get_selectable_option_from_list, print_jump_line, print_message,
};

pub fn new_group_debt_menu(group_debts: &mut Vec<GroupDebt>) {
  clear_terminal();
  print_message("Give a name to your group debt:");
  let name = get_input();

  group_debts.push(GroupDebt {
    name: name,
    friends: vec![],
    money_mount: 0.0,
  });

  let last_index = group_debts.len() - 1;

  add_how_much_money_mount_menu(group_debts.get_mut(last_index).unwrap())
}

fn add_how_much_money_mount_menu(group_debt: &mut GroupDebt) {
  //let first_group_debt_name: &str = group_debts.get(0).unwrap().name.as_str();
  //print_message(first_group_debt_name);
  print_message("How much money mount do spent?");
  let money_mount: f32 = get_input().parse().unwrap();

  //edit group_debt
  //first_group_debt.money_mount = money_mount;
  group_debt.money_mount = money_mount;

  //println!("{first_group_debt:?}")

  // call next menu
  add_friends_menu(group_debt);
}

fn add_friends_menu(group_debt: &mut GroupDebt) {
  print_message("Who was with you?");
  let friend_name = get_input();

  let new_friend = Friend {
    name: friend_name,
    paid_debt: false,
  };

  group_debt.friends.push(new_friend);

  //println!("{first_group_debt:?}")

  // call next menu
  ask_add_other_friend_menu(group_debt);
}

fn ask_add_other_friend_menu(group_debt: &mut GroupDebt) {
  print_message("Do you want to add other friend?");
  let options = vec!["yes", "no"];
  let selected_option = get_selectable_option_from_list(&options).unwrap();

  if selected_option == 0 {
    add_friends_menu(group_debt);
  }
}

pub fn group_debt_menu(group_debt: &mut GroupDebt) {
  let group_name = group_debt.name.as_str();

  print_message(group_name);
  print_message(format!("Remaining debt: {}", group_debt.calculate_remaining_debt()).as_str());
  print_jump_line();

  // Show friends that already paid
  if group_debt.get_friend_already_paid().len() > 0 {
    print_message("Friends already paid:");
    for friend in &group_debt.friends {
      if friend.paid_debt {
        print_message(friend.name.as_str());
      }
    }
    print_jump_line();
  }

  print_message("Friends that have not paid yet:");
  for friend in &group_debt.friends {
    if !friend.paid_debt {
      print_message(friend.name.as_str());
    }
  }
  print_jump_line();

  print_message("What do you want to do?");
  let options = ["A friend wants to pay me", "return to group debts list "].to_vec();
  let selected_option: usize = get_selectable_option_from_list(&options).unwrap();

  match selected_option {
    0 => ask_which_friend_paid_menu(group_debt),
    _ => {}
  }
}

fn ask_which_friend_paid_menu(source_group_debt: &mut GroupDebt) {
  clear_terminal();

  println!("{:?}", source_group_debt);

  let friends = &mut source_group_debt.friends;

  print_message("Which friend paid you?");
  let clone_friends_for_options = friends.clone();

  let mut options = clone_friends_for_options
    .iter()
    .filter(|friend| !friend.paid_debt)
    .map(|friend| friend.name.as_str())
    .collect::<Vec<&str>>();

  let return_option = "return to group debt details";

  options.push(return_option);
  let last_option_index = options.len() - 1;

  let selected_option = get_selectable_option_from_list(&options).unwrap();
  let selected_friend_name = options.get(selected_option).unwrap();

  if last_option_index == selected_option {
    return group_debt_menu(source_group_debt);
  }

  let iterable_found_friends = friends
    .iter_mut()
    .filter(|friend| selected_friend_name.eq(&friend.name));

  if let Some(found_friend) = iterable_found_friends.last() {
    found_friend.paid_debt = true;
  }

  ask_which_friend_paid_menu(source_group_debt);
}
