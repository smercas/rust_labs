mod p1;
mod p2;
mod p3;
mod p4;
mod p5;

fn main() {
    p1::main();
    p2::main();
    match p3::main() {
        Ok(_) => {
            println!("No problem yet");
        },
        Err(error) => {
            match error {
                p3::Error::Overflow => {
                    panic!("Overflow pt 2");
                },
            }
        }
    }
    match p4::main() {
        Ok(_) => {
            println!("No problem yet pt 2");
        },
        Err(error) => {
            p4::print_error(error);
        }
    }
    p5::main();
}
