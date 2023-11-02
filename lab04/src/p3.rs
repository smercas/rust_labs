use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
}

fn replace_abbreviations(original: &str, abbreviations: &[(&str, &str)]) -> String{
  let mut result: String = String::with_capacity(original.len());
  result.push_str(original);
  for abbreviation in abbreviations {
    result = result.replace(abbreviation.0, abbreviation.1);
  }
  result
}

pub fn main(to_replace: &str) -> Result<String, Error> {
  let abbreviations: [(&str, &str); 4] = [("pt", "pentru"), ("ptr", "pentru"), ("dl", "domnul"), ("dna", "doamna")];
  println!("Original:\n{}", to_replace);
  println!("REplaced:\n{}", replace_abbreviations(to_replace, &abbreviations));
  Ok(replace_abbreviations(to_replace, &abbreviations))
}
