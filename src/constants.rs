use std::env;

pub fn get_db_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
  let path = env::current_dir()?.join("./database.db");
  Ok(path)
}
