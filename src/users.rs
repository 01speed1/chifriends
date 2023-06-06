use rusqlite::Connection;

pub struct Users {
  pub id: i32,
  pub username: String,
}

pub enum AddUserResult {
  Created,
  Exists,
  Error,
}

pub trait Databaseble {
  fn add(&self) -> Result<AddUserResult, Box<dyn std::error::Error>>;
}

impl Databaseble for Users {
  pub fn add(username: String) -> Result<AddUserResult, Box<dyn std::error::Error>> {
    let connection = Connection::open_in_memory()?;

    connection.execute(
      "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL
      )",
      [],
    )?;

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
}
