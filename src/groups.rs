use crate::constants::get_db_path;
use rusqlite::{types::ToSql, Connection};

#[derive(Debug)]
pub struct Group {
  id: i32,
  pub user_id: i32,
  pub name: String,
}

impl Group {
  pub fn add(user_id: i32, name: &str) {
    if Group::table_exists("groups").unwrap() {
      if !Group::exists(user_id, name).unwrap() {
        let group = Group {
          id: 0,
          user_id,
          name: name.to_string(),
        };

        group.create().unwrap();
      }
    } else {
      Group::create_table().unwrap();

      let group = Group {
        id: 0,
        user_id,
        name: name.to_string(),
      };

      group.create().unwrap();
    }
  }

  pub fn already_added(user_id: i32, name: &str) -> bool {
    if Group::table_exists("groups").unwrap() {
      if Group::exists(user_id, name).unwrap() {
        return true;
      }
    }

    false
  }
}

trait Databaseble {
  fn create_table() -> Result<(), Box<dyn std::error::Error>>;

  fn table_exists(table_name: &str) -> Result<bool, Box<dyn std::error::Error>>;

  fn exists(user_id: impl ToSql, name: impl ToSql) -> Result<bool, Box<dyn std::error::Error>>;

  fn query(user_id: impl ToSql, name: impl ToSql) -> Result<Group, Box<dyn std::error::Error>>;

  fn create(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Databaseble for Group {
  fn create_table() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    connection.execute(
      "CREATE TABLE IF NOT EXISTS groups (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL
            )",
      [],
    )?;

    if connection.close().is_err() {
      return Err("Error closing connection".into());
    };

    Ok(())
  }

  fn table_exists(table_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let table_exists = connection
      .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")?
      .exists([table_name])?;

    if connection.close().is_err() {
      return Err("Error closing connection".into());
    };

    Ok(table_exists)
  }

  fn exists(user_id: impl ToSql, name: impl ToSql) -> Result<bool, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let mut statement = connection.prepare("SELECT 1 FROM groups WHERE user_id=? AND name=?")?;

    let mut rows = statement.query([&user_id as &dyn ToSql, &name as &dyn ToSql])?;

    let row = rows.next()?;

    Ok(row.is_some())
  }

  fn query(user_id: impl ToSql, name: impl ToSql) -> Result<Group, Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    let mut statement = connection.prepare("SELECT * FROM groups WHERE user_id=? AND name=?")?;

    let mut rows = statement.query([&user_id as &dyn ToSql, &name as &dyn ToSql])?;

    let row = rows.next()?;

    match row {
      Some(row) => Ok(Group {
        id: row.get(0)?,
        user_id: row.get(1)?,
        name: row.get(2)?,
      }),
      None => Err("No group found".into()),
    }
  }

  fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(get_db_path()?)?;

    connection.execute(
      "INSERT INTO groups (user_id, name) VALUES (?1, ?2)",
      &[&self.user_id as &dyn ToSql, &self.name as &dyn ToSql],
    )?;

    if connection.close().is_err() {
      return Err("Error closing connection".into());
    };

    Ok(())
  }
}
