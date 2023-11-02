use thiserror::Error;

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;

#[derive(Error, Debug)]
enum Error {
    #[error("{}", 0)]
    P1Error(#[from] std::io::Error),
    #[error("{}", 0)]
    P2Error(#[from] p2::Error),
    #[error("{}", 0)]
    P3Error(#[from] p3::Error),
    #[error("{}", 0)]
    P4Error(#[from] p4::Error)
}

fn main() -> Result<(), Error> {
    {
        //p1
        let (longest_bytes, longest_chars): (String, String) = p1::main().unwrap_or_default();
        println!("\"{longest_bytes}\"\nbytes:\t{}\nchars:\t{}\n\"{longest_chars}\"\nbytes:\t{}\nchars:\t{}",
            longest_bytes.len(), longest_bytes.chars().count(), longest_chars.len(), longest_chars.chars().count());
    }
    {
        let encrypted = p2::main("text_file_ascii.txt")?;
        println!("\nEncrypted:\n{}", encrypted);
        // let encrypted = p2::main("text_file.txt")?;
        // println!("\nEncrypted:\n{}", encrypted);
    }
    {
        let text: &str = "Am fost la dl Matei pt că m-a invitat cu o zi înainte";
        let no_abbreviations: String = p3::main(text)?;
        println!("\n{}\n=>\n{}", text, no_abbreviations);
    }
    {
        p4::main()?;
    }
    Ok(())
}
