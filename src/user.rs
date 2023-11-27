use crate::group_debt::GroupDebt;

#[derive(Debug)]
pub struct User {
  pub group_debts: Vec<GroupDebt>,
}
