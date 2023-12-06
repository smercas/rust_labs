use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file = std::fs::read_to_string("file.txt")?;
    let separated_file: String = file.chars()
                                     .map(|x| {
                                        if x.is_ascii_punctuation() {
                                            return ' ';
                                        } else {
                                            return x;
                                        }
                                     }).collect::<String>()
                                     .to_lowercase();
    let mut map: HashMap<&str, usize> = HashMap::new();
    for word in separated_file.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    let mut max_length: usize = 0;
    let mut elements: Vec<(&str, usize)> = map.into_iter()
                                              .inspect(|e: &(&str, usize)| { max_length = std::cmp::max(max_length, e.0.chars().count()); } )
                                              .collect::<Vec<(&str, usize)>>();
    elements.sort_by(|a: &(&str, usize), b: &(&str, usize)| { if a.1 == b.1 { return a.0.cmp(&b.0); } else { return b.1.cmp(&a.1); } });
    let _ = elements.iter()
                    .for_each(|e| { println!("{:max_length$} => {}", e.0, e.1); });
    Ok(())
}
