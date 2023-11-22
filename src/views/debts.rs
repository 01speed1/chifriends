use crate::constants::get_db_path;
use rusqlite::{types::ToSql, Connection};

#[derive(Debug)]
pub struct Debt {
  id: i32,
  pub group_id: i32,
  pub name: String,
  pub amount: f64,
}
