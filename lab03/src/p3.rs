pub enum Error {
    Overflow,
}

fn checked_addition(a: u32, b: u32) -> Result<u32, Error> {
    let result: u64 = a as u64 + b as u64;
    if result > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok(result as u32)
}

fn checked_multiplication(a: u32, b: u32) -> Result<u32, Error> {
    let result: u64 = a as u64 * b as u64;
    if result > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok(result as u32)
}

pub fn main() -> Result<(), Error> {
    println!("{}", checked_addition(2, 10)?);
    println!("{}", checked_addition(65535, 65535)?);
    println!("{}", checked_multiplication(2, 10)?);
    println!("{}", checked_multiplication(65535, 65536)?);
    // println!("{}", checked_multiplication(65536, 65536)?);
    Ok(())
}
