use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  // #[error("{} is not an ascii character", 0)]
  // NotAscii(char),
  #[error("{}", 0)]
  IoError(#[from] std::io::Error)
}

pub fn main() -> Result<(), Error> {
  let file: String = std::fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts")?;
  for line in file.lines() {
    if !line.starts_with('#') && !line.is_empty() {
      let mut contents = line.split_ascii_whitespace();
      let ip = contents.next().unwrap_or("___.___.___.___");
      let name = contents.next().unwrap_or("Default_Name");
      println!("{} => {}", ip, name);
    }
  }
  Ok(())
}
