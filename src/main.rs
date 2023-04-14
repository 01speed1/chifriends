#[derive(Debug, Clone)]
struct Friend {
  name: String,
}

#[derive(Debug)]
struct Group {
  name: String,
  friends: Vec<Friend>,
}

impl Group {
  pub fn new(name: String) -> Self {
    Group {
      name,
      friends: vec![],
    }
  }

  fn add_friend(&mut self, friend: Friend) {
    self.friends.push(friend)
  }

  pub fn add_friend_unique(&mut self, friend: Friend) {
    let friend_exists = self.friends.iter().any(|f| f.name == friend.name);
    if !friend_exists {
      self.add_friend(friend)
    }
  }
}

type DebtGroup = Vec<Debt>;

#[derive(Debug)]
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

#[derive(Debug)]
struct Bill {
  payer: Friend,
  service_name: String,
  group: Group,
  debt_group: DebtGroup,
}

impl Bill {
  pub fn new(service_name: String, payer: Friend, group: Group, amount: u32) -> Self {
    let friends_count = group.friends.len();
    let amount_splitted: u32 = amount / friends_count as u32;

    let debt_group: DebtGroup = group
      .friends
      .iter()
      .map(|friend| Debt::new(friend.clone(), amount_splitted))
      .collect::<DebtGroup>();

    Self {
      payer,
      service_name,
      group,
      debt_group,
    }
  }

  pub fn add_friend(&mut self, friend: Friend) {
    self.group.add_friend(friend);
    let friends_count = self.group.friends.len();
    let amount_splitted: u32 =
      self.debt_group.iter().fold(0, |acc, debt| acc + debt.value) / friends_count as u32;

    self.debt_group = self
      .group
      .friends
      .iter()
      .map(|friend| Debt::new(friend.clone(), amount_splitted))
      .collect::<DebtGroup>()
  }

  pub fn pay_partial_debt(&mut self, friend: Friend, value: u32) -> bool {
    let debt = self
      .debt_group
      .iter_mut()
      .find(|debt| debt.friend.name == friend.name);

    match debt {
      Some(debt) => {
        debt.payed_value += value;
        if debt.payed_value >= debt.value {
          debt.is_paid = true;
        }
        true
      }
      None => false,
    }
  }

  //TODO: think better
  pub fn pay_debt(&mut self, friend: Friend) -> bool {
    let mut debt = self
      .debt_group
      .iter_mut()
      .find(|debt| debt.friend.name == friend.name);

    match debt {
      Some(debt) => {
        debt.payed_value = debt.value;
        debt.is_paid = true;
        true
      }
      None => false,
    }
  }
}

fn main() {
  println!("Hello, world!");
}
