#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Invalid line format: {}\n", 0)]
  LineFormatError(String),
  #[error("Invalid identifier: {}\n", 0)]
  InvalidIdentifier(String),
  #[error("Invalid value: {}\n", 0)]
  InvalidValue(String),
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
    let Some((_, contents_and_closing_curly_bracket)) = line.split_once('{') else {
      return Err(Error::LineFormatError(line.to_string()));
    };
    let Some((line_contents, _)) = contents_and_closing_curly_bracket.split_once('}') else {
      return Err(Error::LineFormatError(line.to_string()));
    };
    let names_and_fields: Vec<&str> = line_contents.split(',').collect();
    if names_and_fields.len() != 3 {
      return Err(Error::LineFormatError(line.to_string()));
    }
    let mut name: Option<String> = None;
    let mut phone_number: Option<String> = None;
    let mut age: Option<usize> = None;
    for name_and_field in names_and_fields {
      let Some((guarded_identifier, guarded_value)) = name_and_field.split_once(':') else {
        return Err(Error::LineFormatError(line.to_string()));
      };
      let Some((_, weakly_guarded_identifier)) = guarded_identifier.split_once('\"') else {
        return Err(Error::InvalidIdentifier(guarded_identifier.to_string()));
      };
      let Some((identifier, _)) = weakly_guarded_identifier.split_once('\"') else {
        return Err(Error::InvalidIdentifier(weakly_guarded_identifier.to_string()));
      };
      match identifier {
        "name" => {
          let Some((_, weakly_guarded_value)) = guarded_value.split_once('\"') else {
            return Err(Error::InvalidValue(guarded_value.to_string()));
          };
          let Some((value, _)) = weakly_guarded_value.split_once('\"') else {
            return Err(Error::InvalidValue(weakly_guarded_value.to_string()));
          };
          name = Some(value.to_string());
        }
        "phone" => {
          let Some((_, weakly_guarded_value)) = guarded_value.split_once('\"') else {
            return Err(Error::InvalidValue(guarded_value.to_string()));
          };
          let Some((value, _)) = weakly_guarded_value.split_once('\"') else {
            return Err(Error::InvalidValue(weakly_guarded_value.to_string()));
          };
          phone_number = Some(value.to_string());
        }
        "age" => {
          let Some((_, weakly_guarded_value)) = guarded_value.split_once(' ') else {
            return Err(Error::InvalidValue(guarded_value.to_string()));
          };
          let Some((value, _)) = weakly_guarded_value.split_once(' ') else {
            return Err(Error::InvalidValue(weakly_guarded_value.to_string()));
          };
          age = Some(usize::from_str_radix(value, 10)?);
        }
        _ => {
          return Err(Error::InvalidIdentifier(identifier.to_string()));
        }
      }
    }
    match (name, phone_number, age) {
      (Some(n), Some(pn), Some(a)) => {
        person = Person {
          name: n,
          phone_number: pn,
          age: a
        }
      }
      _ => { return Err(Error::LineFormatError(line.to_string())); }
    }
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
