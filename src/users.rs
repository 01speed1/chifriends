use crate::{constants::get_db_path, groups::Group};
use rusqlite::{Connection, Error::QueryReturnedNoRows};

#[derive(Debug)]
pub struct User {
  id: i32,
  pub username: String,
  pub recorded: bool,
}

impl User {
  pub fn new(username: String) {
    let new_user = User {
      id: 0,
      username,
      recorded: false,
    };

    new_user.create().unwrap();
  }

  fn get() -> Self {
    Self::query().unwrap()
  }

  pub fn already_exists() -> bool {
    let exists = Self::exists();
    if exists.is_err() {
      println!("Error: {}", exists.unwrap_err());
      return false;
    } else {
      exists.unwrap()
    }
  }

  pub fn get_username() -> String {
    Self::get().username
  }

  pub fn add_group(group_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Group::already_added(Self::get().id, group_name) {
      Group::add(Self::get().id, group_name);
    } else {
      println!("Group {} already added", &group_name);
    }

    Ok(())
  }
}

pub enum AddUserResult {
  Created,
  Error,
  Exists,
}

trait Databaseble {
  fn create_table() -> Result<(), Box<dyn std::error::Error>>;

  fn table_exists(table_name: &str) -> Result<bool, Box<dyn std::error::Error>>;

  fn exists() -> Result<bool, Box<dyn std::error::Error>>;

  fn create(&self) -> Result<AddUserResult, Box<dyn std::error::Error>>;

  fn query() -> Result<User, Box<dyn std::error::Error>>;
}

impl Databaseble for User {
  fn create_table() -> Result<(), Box<dyn std::error::Error>> {
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

  fn table_exists(table_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let table_exists = connection
      .prepare("SELECT 1 FROM sqlite_master WHERE type='table' AND name=? LIMIT 1")?
      .exists(&[table_name])?;

    if connection.close().is_err() {
      return Err("Error closing connection in 'table exist' method".into());
    }
    Ok(table_exists)
  }

  fn exists() -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    if !Self::table_exists("users")? {
      return Ok(false);
    }

    let query = "SELECT COUNT(*) FROM users;";
    let found_users = connection.query_row(query, [], |row| row.get(0))?;

    let response = match found_users {
      0 => Ok(false),
      1 => Ok(true),
      _ => Err("Error in 'exists' method, exists more that 1 user".into()),
    };

    if connection.close().is_err() {
      return Err("Error closing connection in 'exists' method".into());
    }

    response
  }

  fn create(&self) -> Result<AddUserResult, Box<dyn std::error::Error>> {
    let username = self.username.clone();

    let connection = Connection::open(get_db_path()?)?;

    if !Self::table_exists("users")? {
      Self::create_table()?;
    }

    println!("Checking if user exists {}", Self::exists()?);

    if Self::exists()? {
      return Ok(AddUserResult::Exists);
    }

    connection.execute("INSERT INTO users (username) VALUES (?1)", [username])?;

    if connection.close().is_err() {
      return Ok(AddUserResult::Error);
    };

    Ok(AddUserResult::Created)
  }

  fn query() -> Result<User, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let found_user = connection.query_row("SELECT * FROM users LIMIT 1", [], |user| {
      Ok(User {
        id: user.get(0)?,
        username: user.get(1)?,
        recorded: true,
      })
    });

    let response = match found_user {
      Ok(user) => Ok(user),
      Err(QueryReturnedNoRows) => Err("No user found".into()),
      Err(e) => Err(e.into()),
    };

    if connection.close().is_err() {
      return Err("Error closing connection in 'get' method".into());
    }

    response
  }
}
