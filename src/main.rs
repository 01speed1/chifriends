use crossterm::terminal::{Clear, ClearType};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;
use std::usize;

// Terminal option
fn clear_terminal() -> io::Result<()> {
  let mut stdout = io::stdout();
  crossterm::queue!(
    stdout,
    Clear(ClearType::All),
    crossterm::cursor::MoveTo(0, 0)
  )?;
  stdout.flush()?;
  Ok(())
}

fn get_option_menu_number() -> i32 {
  let mut buffer = String::new();
  std::io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to get value");

  buffer.trim().parse::<i32>().unwrap()
}

fn get_input() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

//menu interface options
fn load_main_menu(user: &mut User) -> ! {
  loop {
    println!("Chifriends");

    if !user.has_groups() {
      println!("You don't have groups, add one.");
      println!("What is the name of your new group?:");

      let group_name = get_input();
      let _choose_group = user.add_group_unique(&group_name).unwrap();
      user.set_current_group(0).unwrap();
      clear_terminal().unwrap();
      continue;
    }

    println!("Your Groups, select one:");
    for (index, group) in user.groups.iter().enumerate() {
      print!("{}: {}", index + 1, group.name)
    }
    println!("");
    println!("More options:");
    println!("0: Create new group");

    println!("");
    println!("Write a number to select a option: ");

    let input = get_option_menu_number();

    match input {
      0 => {
        println!("What is the name of your new group?:");
        let group_name = get_input();
        let _choose_group = user.add_group_unique(&group_name).unwrap();
        user.set_current_group(user.groups.len() - 1).unwrap();
      }
      index => {
        let parsed_index: usize = (index - 1) as usize;
        match user.set_current_group(parsed_index).is_some() {
          true => {
            clear_terminal().unwrap();
            load_group_menu(user).unwrap();
          }
          false => {
            clear_terminal().unwrap();
            println!("Invalid input {} please, try again!!!", index);
            println!("");

            continue;
          }
        }
      }
    }
  }
}

fn load_menu_add_friend(group: &mut Group) {
  println!("What is the name of your new friend?:");
  let friend_name = get_input();

  let _choose_friend = group.add_friend_unique(Friend {
    name: friend_name.to_string(),
  });

  if _choose_friend.is_some() {
    println!("Friend added '{}'", friend_name);
  } else {
    println!("Friend already exists");
  }
  println!("");
}

fn load_group_menu(user: &mut User) -> Result<(), String> {
  let group = user.get_current_group();

  loop {
    println!("this is your group menu for '{}'.", &group.name.trim());
    println!("");

    println!("Your friends:");
    if !group.has_friends() {
      println!("You don't have friends in this group, add one.");
      println!("");
    }

    for (index, friend_name) in group.get_friends_names().iter().enumerate() {
      println!("{}: {}", index + 1, friend_name);
    }
    println!("");

    println!("Your bills in this group:");
    if !group.has_bills() {
      println!("You don't have bills, add one.");
      println!("");
    }

    println!("Your options:");
    println!("1: Add friend");
    println!("2: Add bill");
    println!("0: Back to main menu");
    println!("");

    println!("Write a number to select a option: ");
    let option = get_option_menu_number();
    match option {
      1 => {
        clear_terminal().unwrap();
        load_menu_add_friend(group);
        user.auto_save().unwrap();
        continue;
      }
      0 => {
        clear_terminal().unwrap();
        load_main_menu(user);
      }
      other => {
        println!("Invalid input {} please, try again!!!", other);
        println!("");

        continue;
      }
    }
  }
}

// ============================================================================ //

// business logic
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Friend {
  name: String,
}

type Friends = Vec<Friend>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Group {
  name: String,
  friends: Friends,
  bills: Bills,
}

impl Group {
  pub fn new(name: String) -> Self {
    Group {
      name,
      friends: vec![],
      bills: vec![],
    }
  }

  fn add_friend(&mut self, friend: Friend) {
    self.friends.push(friend);
  }

  pub fn add_friend_unique(&mut self, friend: Friend) -> Option<Friend> {
    let friend_exists = self.friends.iter().any(|f| f.name == friend.name);

    if !friend_exists {
      self.add_friend(friend);
      Some(self.friends.last().unwrap().clone())
    } else {
      None
    }
  }

  fn has_friends(&self) -> bool {
    !self.friends.is_empty()
  }

  fn has_bills(&self) -> bool {
    !self.bills.is_empty()
  }

  // create a function for group to get a names of friends in a vec
  pub fn get_friends_names(&self) -> Vec<String> {
    let mut friends_names: Vec<String> = vec![];
    for friend in &self.friends {
      friends_names.push(friend.name.clone());
    }
    friends_names
  }
}

type DebtGroup = Vec<Debt>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Debt {
  friend: Friend,
  value: u32,
  is_paid: bool,
  payed_value: u32,
}

impl Debt {
  pub fn new(friend: Friend, value: u32) -> Self {
    Self {
      friend,
      value,
      is_paid: false,
      payed_value: 0,
    }
  }
}

type Bills = Vec<Bill>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Bill {
  payer: Friend,
  service_name: String,
  group: Group,
  debt_group: DebtGroup,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
  selected_group: Option<Group>,
  groups: Vec<Group>,
}

impl User {
  pub fn new() -> Self {
    Self {
      groups: vec![],
      selected_group: None,
    }
  }

  fn add_group(&mut self, group_name: &str) -> &mut Group {
    let group = Group::new(group_name.to_string());
    self.groups.push(group);
    self.groups.last_mut().unwrap()
  }

  pub fn add_group_unique(&mut self, group_name: &str) -> Result<&mut Group, String> {
    let group_exists = self.groups.iter().any(|g| g.name == group_name);
    if !group_exists {
      self.auto_save()?;
      Ok(self.add_group(group_name))
    } else {
      Err(format!("Group {} already exists", group_name))
    }
  }

  pub fn set_current_group(&mut self, index_group: usize) -> Option<Group> {
    let group = self.groups.get(index_group).clone();

    match group {
      Some(group) => {
        self.auto_save();
        self.selected_group = Some(group.clone());
        self.selected_group.clone()
      }
      None => None,
    }
  }

  pub fn get_current_group(&mut self) -> &Group {
    self.selected_group.as_mut().unwrap()
  }

  pub fn list_groups(&self) -> Vec<String> {
    self.groups.iter().map(|g| g.name.clone()).collect()
  }

  pub fn get_group(&self, group_name: &str) -> Option<&Group> {
    self.groups.iter().find(|g| g.name == group_name)
  }

  pub fn has_groups(&self) -> bool {
    !self.groups.is_empty()
  }

  pub fn auto_save(&self) -> Result<(), String> {
    let json = serde_json::to_string(&self).unwrap();
    let mut file = File::create("user.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    Ok(())
  }

  pub fn load() -> Result<Self, String> {
    if !Path::new("user.json").exists() {
      return Ok(Self::new());
    }

    let mut file = match File::open("user.json") {
      Ok(file) => file,
      Err(error) => return Err(format!("Failed to open file: {}", error)),
    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
      return Err(format!("Failed to read file: {}", error));
    }

    let user: User = match serde_json::from_str(&contents) {
      Ok(user) => user,
      Err(error) => return Err(format!("Failed to parse JSON: {}", error)),
    };

    Ok(user)
  }
}

fn main() {
  clear_terminal();
  let user = User::load();
  load_main_menu(&mut user.unwrap());
}
