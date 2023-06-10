// Mods
mod constants;
mod users;

// Imports
use users::User;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let _main_user: User = users::User::new("main_user")?;

  Ok(())
}
