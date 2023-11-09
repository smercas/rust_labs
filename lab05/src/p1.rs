#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Invalid line format: {}\nNumber of elements:{}\n", 0, 1)]
  LineFormatError(String, usize),
  #[error("{}", 0)]
  IoError(#[from] std::io::Error),
  #[error("{}", 0)]
  ParseIntError(#[from] std::num::ParseIntError)
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Person {
  name: String,
  phone_number: String,
  age: usize,
}

pub fn main(path : &str) -> Result<(Person, Person), Error> {
  let file: String = std::fs::read_to_string(path)?;
  let mut youngest: Option<Person> = None;
  let mut oldest: Option<Person> = None;
  let mut person: Person;
  for line in file.lines() {
    let line_contents: Vec<&str> = line.split(',').collect();
    if line_contents.len() != 3 {
      return Err(Error::LineFormatError(line.to_string(), line_contents.len()));
    }
    person = Person {
      name: line_contents[0].to_string(),
      phone_number: line_contents[1].to_string(),
      age: usize::from_str_radix(line_contents[2], 10)?
    };
    match (youngest.as_mut(), oldest.as_mut()) {
      (None, None) => {
        oldest = Some(person.clone());
        youngest = Some(person);
      }
      (Some(y), Some(o)) => {
        if y.age > person.age {
          // youngest = Some(person);
          *y = person;
        } else if o.age < person.age {
          // oldest = Some(person);
          *o = person;
        }
      }
      _ => { panic!("one element was initialised but the other wasn't"); }
    }
  }
  Ok((youngest.unwrap(), oldest.unwrap()))
}
