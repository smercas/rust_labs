#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", 0)]
    IoError(#[from] std::io::Error)
}

pub fn main() -> Result<(String, String), Error> {
    let file: String = std::fs::read_to_string("text_file.txt")?;
    let mut longest_bytes: &str = "";
    let mut longest_chars: &str = "";
    for line in file.lines() {
        if line.len() > longest_bytes.len() {
            longest_bytes = line;
        }
        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }
    Ok((longest_bytes.to_string(), longest_chars.to_string()))
}
