use crate::group_debt::GroupDebt;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  pub group_debts: Vec<GroupDebt>,
}

impl User {
  pub fn save_to_file(&self) -> std::io::Result<()> {
    let json = serde_json::to_string(&self).unwrap();
    let mut file = File::create("user.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
  }

  pub fn load_from_file() -> std::io::Result<Self> {
    let mut file = File::open("user.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
      Ok(User {
        group_debts: Vec::new(),
      })
    } else {
      let user: User = serde_json::from_str(&contents).unwrap();
      Ok(user)
    }
  }
}
