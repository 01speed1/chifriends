use crate::friend::Friend;

#[derive(Debug)]
pub struct GroupDebt {
  pub name: String,
  pub friends: Vec<Friend>,
  pub money_mount: f32,
}

impl GroupDebt {
  pub fn calculate_remaining_debt(&self) -> f32 {
    let mut remaining_debt: f32 = self.money_mount;
    for friend in &self.friends {
      if friend.paid_debt {
        remaining_debt -= self.money_mount / self.friends.len() as f32;
      }
    }
    remaining_debt
  }

  pub fn get_friend_already_paid(&self) -> Vec<&Friend> {
    let mut friends_already_paid: Vec<&Friend> = vec![];
    for friend in &self.friends {
      if friend.paid_debt {
        friends_already_paid.push(friend);
      }
    }
    friends_already_paid
  }
}
