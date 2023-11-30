use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
  pub name: String,
  pub paid_debt: bool,
}
