use create::entities::users::User;
use rusqlite::Connection;

pub enum AddUserResponse {
  Exist,
  Created,
}

pub impl User {
  pub fn add_user(username: String) -> Result<AddUserResponse> {
    let connection = Connection::open_in_memory()?;

    connection.execute(
      "CREATE TABLE IF NOT EXISTS Users (
          id SERIAL PRIMARY KEY,
          username VARCHAR(255)
      );",
      [],
    )?;

    let mut statement = connection.prepare("SELECT * FROM USERS")?;

    let mut found_users = statement.query([])?;

    while let Some(user) = found_users.next()? {
      println!("Found user {:?}", user.get::<_, String>(1)?);
      return Ok(AddUserResponse::Exist);
    }

    let app_user = User { id: 0, username };

    let query_result = connection.execute(
      "INSERT INTO users (username) VALUES (?1)",
      (&app_user.username),
    );

    let a = Vec::new();
  }

  pub fn get_user(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let user_iter = stmt.query_map(&[], |row| {
      Ok(User {
        id: row.get(0)?,
        name: row.get(1)?,
        email: row.get(2)?,
      })
    })?;

    let mut users = Vec::new();
    for user in user_iter {
      users.push(user.unwrap());
    }
    Ok(users)
  }
}
