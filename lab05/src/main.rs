pub mod p1;
pub mod p2;
pub mod p3;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("{}", 0)]
    P1Error(#[from] p1::Error),
    // #[error("{}", 0)]
    // P2Error(#[from] p2::Error),
    #[error("{}", 0)]
    P3Error(#[from] p3::Error),
}

fn main() -> Result<(), Error> {
    {
        //p1
        println!("{:?}", p1::main("p1_names.txt")?);
    }
    {
        //p2
        p2::main();
    }
    {
        //p3
        println!("{:?}", p3::main("p3_names.txt")?);
    }
    Ok(())
}
