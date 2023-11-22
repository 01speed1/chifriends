use crate::constants::get_db_path;
use rusqlite::{types::ToSql, Connection};

#[derive(Debug)]
pub struct Friend {
  id: i32,
  pub user_id: i32,
  pub name: String,
}

trait Databaseble {
  fn create_table() -> Result<(), Box<dyn std::error::Error>>;

  fn table_exists(table_name: &str) -> Result<bool, Box<dyn std::error::Error>>;

  fn exists(user_id: impl ToSql, name: impl ToSql) -> Result<bool, Box<dyn std::error::Error>>;

  fn query(user_id: impl ToSql, name: impl ToSql) -> Result<Friend, Box<dyn std::error::Error>>;

  fn create(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Databaseble for Friend {
  fn create_table() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    connection.execute(
      "CREATE TABLE IF NOT EXISTS friends (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL
            )",
      [],
    )?;

    Ok(())
  }

  fn table_exists(table_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    todo!()
  }

  fn exists(user_id: impl ToSql, name: impl ToSql) -> Result<bool, Box<dyn std::error::Error>> {
    todo!()
  }

  fn query(user_id: impl ToSql, name: impl ToSql) -> Result<Friend, Box<dyn std::error::Error>> {
    todo!()
  }

  fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
  }
}
