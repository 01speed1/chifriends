use crate::constants::get_db_path;
use rusqlite::Connection;

#[derive(Debug)]
pub struct User {
  pub id: i32,
  pub username: String,
}

impl User {
  pub fn new(username: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let new_user = User {
      id: 0,
      username: username.to_string(),
    };

    new_user.add()?;

    Ok(new_user)
  }
}

pub enum AddUserResult {
  Created,
  Exists,
  Error,
  TableNotFound,
}

pub trait Databaseble {
  fn table_exists(&self, table_name: &str) -> Result<bool, Box<dyn std::error::Error>>;

  fn exists(&self) -> Result<bool, Box<dyn std::error::Error>>;

  fn add(&self) -> Result<AddUserResult, Box<dyn std::error::Error>>;

  fn get(&self) -> Result<User, Box<dyn std::error::Error>>;
}

impl Databaseble for User {
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

  fn add(&self) -> Result<AddUserResult, Box<dyn std::error::Error>> {
    let username = self.username.clone();

    let connection = Connection::open(get_db_path()?)?;

    if !self.table_exists("users")? {
      return Ok(AddUserResult::TableNotFound);
    }

    let found_users = connection.execute("SELECT * FROM users", [])?;

    if found_users > 0 {
      return Ok(AddUserResult::Exists);
    }

    connection.execute("INSERT INTO users (username) VALUES (?1)", [username])?;

    if connection.close().is_err() {
      return Ok(AddUserResult::Error);
    };

    Ok(AddUserResult::Created)
  }

  fn get(&self) -> Result<User, Box<dyn std::error::Error>> {
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
