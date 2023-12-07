use std::fs::File;
use std::path::Path;

mod friend;
mod group_debt;
mod interface;
mod menu;
mod user;
mod view;

//external uses
use iced::theme::Button::Destructive;
use iced::widget::{button, column, horizontal_space, row, text, text_input, vertical_space};
use iced::{Element, Length, Sandbox, Settings};

//use crate::menu::main_menu;
use crate::friend::Friend;
use crate::group_debt::GroupDebt;
use crate::user::User;

fn main() -> iced::Result {
  let path = Path::new("user.json");
  if !path.exists() {
    File::create(path).expect("Failed to create file");
  }

  //for terminal version
  //let mut user = User::load_from_file().unwrap();

  loop {
    let _ = MainApp::run(Settings::default());

    //for terminal version
    //main_menu(&mut user);
  }
}

pub enum AppView {
  Main,
  NewGroupDebt,
  GroupDebts,
  GroupDebt,
}

pub struct State {
  group_debt: GroupDebt,
  friend_name: String,
  group_debt_selected: usize,
}

pub struct MainApp {
  debug: bool,
  view_state: AppView,
  user: User,
  state: State,
}

#[derive(Debug, Clone)]
pub enum Message {
  AddNewGroup,
  ReturnToMain,
  NewFriendAdded,
  NewGroupDebtSaved,
  FriendPaid(usize),
  GroupDebtSelected(usize),
  NewFriendsNameChanged(String),
  NewGroupDebtNameChanged(String),
  NewGroupDebtMoneyMountChanged(String),
}

impl Sandbox for MainApp {
  type Message = Message;

  fn new() -> Self {
    let user = User::load_from_file().unwrap();

    Self {
      debug: false,
      view_state: AppView::Main,
      user: user,
      state: State {
        group_debt: GroupDebt {
          name: "".to_owned(),
          friends: vec![],
          money_mount: 0.0,
        },
        friend_name: "".to_owned(),
        group_debt_selected: 0,
      },
    }
  }

  fn title(&self) -> String {
    String::from("Chifriends")
  }

  fn update(&mut self, event: Message) {
    match event {
      Message::AddNewGroup => {
        self.view_state = AppView::NewGroupDebt;
      }
      Message::NewGroupDebtNameChanged(new_name) => {
        self.state.group_debt.name = new_name;
      }
      Message::NewGroupDebtMoneyMountChanged(new_money_mount) => {
        self.state.group_debt.money_mount = new_money_mount.parse().unwrap_or(0.0);
      }
      Message::NewGroupDebtSaved => {
        self.user.group_debts.push(self.state.group_debt.clone());
        self.user.save_to_file().unwrap();
        self.view_state = AppView::Main;
      }
      Message::NewFriendsNameChanged(name) => {
        self.state.friend_name = name;
      }
      Message::NewFriendAdded => {
        if self.state.friend_name == "" {
          return;
        }

        self.state.group_debt.friends.push(Friend {
          name: self.state.friend_name.clone(),
          paid_debt: false,
        });
        self.state.friend_name = "".to_owned();
      }
      Message::ReturnToMain => {
        self.view_state = AppView::Main;
      }
      Message::GroupDebtSelected(group_debt_index) => {
        self.state.group_debt_selected = group_debt_index;
        self.view_state = AppView::GroupDebt;
      }
      Message::FriendPaid(friend_index) => {
        self.user.group_debts[self.state.group_debt_selected].friends[friend_index].paid_debt =
          true;
      }
    }
  }

  fn view(&self) -> Element<'_, Self::Message> {
    let MainApp {
      view_state,
      state,
      debug,
      user,
    } = self;

    let title = text("Chifriends").width(Length::Fill).size(50);
    let mut main_column = column![title].padding(35);

    match view_state {
      AppView::Main => {
        if self.user.group_debts.len() == 0 {
          let t1 = text("You don't have group debts yet, create one!")
            .size(20)
            .height(40);
          let add_new_group_button = button("Add new group").on_press(Message::AddNewGroup);

          main_column.push(t1).push(add_new_group_button).into()
        } else {
          let t1 = text("Your group debts:").size(20).height(40);

          main_column = main_column.push(t1);

          for (group_debt_index, group_debt) in self.user.group_debts.iter().enumerate() {
            let button_label = if group_debt.friends.iter().all(|friend| friend.paid_debt) {
              format!("{} Completed", group_debt.name.clone())
            } else {
              group_debt.name.clone()
            };

            let group_debt_button = button(text(button_label))
              .width(Length::Fill)
              .on_press(Message::GroupDebtSelected(group_debt_index));

            main_column = main_column.push(group_debt_button).push(vertical_space(10));
          }

          main_column = main_column.push(vertical_space(10));

          main_column = main_column.width(300);

          main_column = main_column.push(button("Add new group").on_press(Message::AddNewGroup));

          main_column.into()
        }
      }
      AppView::NewGroupDebt => {
        let label_group_debt_name_text = text("Give a name to your group debt:").size(20);
        let input_group_debt_name =
          text_input("Name", &state.group_debt.name).on_input(Message::NewGroupDebtNameChanged);

        main_column = main_column
          .push(label_group_debt_name_text)
          .push(input_group_debt_name)
          .push(vertical_space(20));

        let label_group_debt_money_mount_text = text("How much money mount do spent?").size(20);
        let input_group_debt_money_mount =
          text_input("Money Mount", &state.group_debt.money_mount.to_string())
            .on_input(Message::NewGroupDebtMoneyMountChanged);

        main_column = main_column
          .push(label_group_debt_money_mount_text)
          .push(input_group_debt_money_mount);

        let mut input_friends_row = row![].spacing(30);

        let label_group_debt_friend_name_text = text("Who was with you?").size(20);
        main_column = main_column.push(label_group_debt_friend_name_text);

        let input_group_debt_friend_name =
          text_input("Friend Name", &state.friend_name).on_input(Message::NewFriendsNameChanged);
        let add_group_debt_friend_button = button("Add friend").on_press(Message::NewFriendAdded);

        input_friends_row = input_friends_row
          .push(input_group_debt_friend_name)
          .push(add_group_debt_friend_button);

        main_column = main_column.push(input_friends_row);

        main_column = main_column.push(vertical_space(40));

        let mut buttons_row = row![].spacing(30);

        let return_button = button("Return to main")
          .style(Destructive)
          .on_press(Message::ReturnToMain);

        let save_group_debt_button = button("Save group debt").on_press(Message::NewGroupDebtSaved);

        let friends_list_column = column![text("Added friends:").size(20)];

        let mut friends_list = self
          .state
          .group_debt
          .friends
          .iter()
          .fold(friends_list_column, |column, friend| {
            column.push(text(friend.name.clone()))
          });

        friends_list = friends_list.push(vertical_space(10));

        main_column = main_column.push(friends_list.width(200));

        buttons_row = buttons_row.push(return_button).push(save_group_debt_button);
        main_column.push(buttons_row).into()
      }
      AppView::GroupDebt => {
        let group_debt = &self.user.group_debts[self.state.group_debt_selected];

        let title = text(group_debt.name.clone()).width(Length::Fill).size(50);

        main_column = main_column.push(title);

        let mut friends_list_column = column![text("Friends:").size(20)];

        for (index, friend) in group_debt.friends.iter().enumerate() {
          if friend.paid_debt {
            continue;
          }

          let friend_row = row![
            text(friend.name.clone()),
            horizontal_space(30),
            button(text("Paid")).on_press(Message::FriendPaid(index))
          ];

          friends_list_column = friends_list_column.push(friend_row);
        }

        main_column = main_column.push(friends_list_column);

        let mut friends_list_already_paid_column = column![text("Friends already paid:").size(20)];

        for friend in group_debt.friends.iter() {
          if !friend.paid_debt {
            continue;
          }
          let friends_list_already_paid_row = row![
            text(friend.name.clone()),
            horizontal_space(30),
            text("Already paid")
          ];

          friends_list_already_paid_column =
            friends_list_already_paid_column.push(friends_list_already_paid_row);
        }

        main_column = main_column.push(friends_list_already_paid_column);

        let return_button = button("Return to main")
          .style(Destructive)
          .on_press(Message::ReturnToMain);

        main_column = main_column.push(vertical_space(30)).push(return_button);

        main_column.into()
      }
      _ => main_column.push(text("Nothing yet")).into(),
    }
  }
}
