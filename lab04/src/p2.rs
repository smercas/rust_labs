#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{} is not an ascii character", 0)]
    NotAscii(char),
    #[error("{}", 0)]
    IoError(#[from] std::io::Error)
    }

fn encrypt(c: &mut char) -> Result<(), Error> {
    if !c.is_ascii() {
        return Err(Error::NotAscii(*c));
    }
    for limit in [['a', 'z'], ['A', 'Z']] {
        if *c >= limit[0] && *c <= limit[1] {
            *c = ((*c as u8 - limit[0] as u8 + 13) % 26 + limit[0] as u8) as char;
        }
    }
    Ok(())
}

pub fn main(path: &str) -> Result<String, Error> {
    let file: String = std::fs::read_to_string(path)?;
    let mut newfile: String = String::with_capacity(file.len());
    let mut newerfile: String = String::with_capacity(file.len());
    for mut c in file.chars() {
        encrypt(&mut c)?;
        newfile.push(c);
        encrypt(&mut c)?;
        newerfile.push(c);
    }
    Ok(newfile)
}
