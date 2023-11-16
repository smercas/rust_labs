pub mod p1;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("{}", 0)]
    P1Error(#[from] p1::Error),
    // #[error("{}", 0)]
    // P2Error(#[from] p2::Error),
}

fn main() {
    {
        match p1::main() {
            Ok(_) => {
                //
            },
            Err(error) => {
                println!("{}", error);
            },
        }
    }
}
