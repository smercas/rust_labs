pub enum Error {
    NotAscii,
    NotADigit,
    NotABase16Digit,
    NotALetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, Error> {
    if c >= 'a' && c <= 'z' {
        Ok((c as u8 - 'a' as u8 + 'A' as u8) as char)
    } else if c >= 'A' && c <= 'Z' {
        Ok(c)
    } else {
        Err(Error::NotALetter)
    }
}

fn to_lowercase(c: char) -> Result<char, Error> {
    if c >= 'A' && c <= 'Z' {
        Ok((c as u8 - 'A' as u8 + 'a' as u8) as char)
    } else if c >= 'a' && c <= 'z' {
        Ok(c)
    } else {
        Err(Error::NotALetter)
    }
}

fn print_char(c: char) -> Result<(), Error> {
    if !(c >= '!' && c <= '~') {
        return Err(Error::NotPrintable);
    }
    print!("{}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u8, Error> {
    if !(c >= 0 as char && c <= 127 as char) {
        return Err(Error::NotAscii);
    }
    if c >= '0' && c <= '9' {
        Ok(c as u8 - '0' as u8)
    } else {
        Err(Error::NotADigit)
    }
}

fn char_to_number_hex(c: char) -> Result<u8, Error> {
    if !(c >= 0 as char && c <= 127 as char) {
        return Err(Error::NotAscii);
    }
    if c >= '0' && c <= '9'{
        Ok(c as u8 - '0' as u8)
    } else if c >= 'a' && c <= 'f' {
        Ok(c as u8 - 'a' as u8 + 10)
    } else if c >= 'A' && c <= 'F' {
        Ok(c as u8 - 'A' as u8 + 10)
    } else {
        Err(Error::NotABase16Digit)
    }
}

pub fn print_error(error: Error) {
    match error {
        Error::NotAscii => {
            println!("not ascii");
        },
        Error::NotADigit => {
            println!("not a digit");
        },
        Error::NotABase16Digit => {
            println!("not a base 16 digit");
        },
        Error::NotALetter => {
            println!("not a letter");
        },
        Error::NotPrintable => {
            println!("not printable");
        },
    }
}

pub fn main() -> Result<(), Error> {
    println!("{} => {}", 'a', to_uppercase('a')?);
    println!("{} => {}", 'A', to_uppercase('A')?);
    // println!("{} => {}", '0', to_uppercase('0')?);
    println!("{} => {}", 'b', to_lowercase('b')?);
    println!("{} => {}", 'B', to_lowercase('B')?);
    // println!("{} => {}", '0', to_lowercase('0')?);
    print_char('a')?;
    // print_char(32 as char)?;
    println!();
    println!("{} => {}", '0', char_to_number('0')?);
    // println!("{} => {}", 'a', char_to_number('a')?);
    // println!("{} => {}", 128 as char, char_to_number(128 as char)?);
    println!("{} => {}", '0', char_to_number_hex('0')?);
    println!("{} => {}", 'a', char_to_number_hex('a')?);
    println!("{} => {}", 'F', char_to_number_hex('F')?);
    // println!("{} => {}", 'k', char_to_number_hex('k')?);
    // println!("{} => {}", 128 as char, char_to_number_hex(128 as char)?);
    Ok(())
}
