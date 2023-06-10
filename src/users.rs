use crate::constants::get_db_path;
use rusqlite::Connection;

#[derive(Debug)]
pub struct User {
  id: i32,
  pub username: String,
  pub recorded: bool,
}

impl User {
  pub fn tomporal_new() -> Self {
    User {
      id: 0,
      username: "".to_string(),
      recorded: false,
    }
  }

  pub fn get(&self) -> Self {
    self.query().unwrap()
  }

  pub fn already_exists(&self) -> bool {
    self.exists().unwrap()
  }
}

pub enum AddUserResult {
  Created,
  Error,
  Exists,
}

trait Databaseble {
  fn create_table(&self) -> Result<(), Box<dyn std::error::Error>>;

  fn table_exists(&self, table_name: &str) -> Result<bool, Box<dyn std::error::Error>>;

  fn exists(&self) -> Result<bool, Box<dyn std::error::Error>>;

  fn create(&self) -> Result<AddUserResult, Box<dyn std::error::Error>>;

  fn query(&self) -> Result<User, Box<dyn std::error::Error>>;
}

impl Databaseble for User {
  fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    connection.execute(
      "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL
      )",
      [],
    )?;

    if connection.close().is_err() {
      return Err("Error closing connection in 'create_table' method".into());
    }

    Ok(())
  }

  fn table_exists(&self, table_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let table_exists = connection
      .prepare("SELECT 1 FROM sqlite_master WHERE type='table' AND name=? LIMIT 1")?
      .exists(&[table_name])?;

    if connection.close().is_err() {
      return Err("Error closing connection in 'table exist' method".into());
    }
    Ok(table_exists)
  }

  fn exists(&self) -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    if !self.table_exists("users")? {
      return Ok(false);
    }

    let found_users = connection.execute("SELECT * FROM users", [])?;

    if found_users == 0 {
      return Ok(true);
    }

    if connection.close().is_err() {
      return Err("Error closing connection in 'exists' method".into());
    }

    Ok(false)
  }

  fn create(&self) -> Result<AddUserResult, Box<dyn std::error::Error>> {
    let username = self.username.clone();

    let connection = Connection::open(get_db_path()?)?;

    if self.exists()? {
      return Ok(AddUserResult::Exists);
    }

    if !self.exists()? {
      self.create_table()?;
    }

    connection.execute("INSERT INTO users (username) VALUES (?1)", [username])?;

    if connection.close().is_err() {
      return Ok(AddUserResult::Error);
    };

    Ok(AddUserResult::Created)
  }

  fn query(&self) -> Result<User, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let result = connection.query_row("SELECT * FROM users LIMIT 1", [], |user| {
      Ok(User {
        id: user.get(0)?,
        username: user.get(1)?,
      })
    })?;

    if connection.close().is_err() {
      return Err("Error closing connection in 'get' method".into());
    }

    Ok(result)
  }
}
