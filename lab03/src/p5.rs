fn read_data_from_file(file_path: &str) -> Result<Option<String>, std::io::Error> {
    let mut file: std::fs::File = std::fs::File::open(file_path)?;

    let mut data: String = String::new();
    std::io::Read::read_to_string(&mut file, &mut data)?;

    // Perform something on the data

    if data.is_empty() {
        Ok(None)
    } else {
        Ok(Some(data))
    }
}

pub fn main() {
    let file_path: &str = "data.txt";
    match read_data_from_file(file_path) {
        Ok(Some(result)) => {
            println!("Data: {}", result);
        },
        Ok(None) => {
            println!("No meaningful data in the file.");
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        },
    }
}
