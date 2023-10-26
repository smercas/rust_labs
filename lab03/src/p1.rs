fn is_prime(number: u16) -> bool {
    if number < 2 { return false; }
    if number == 2 { return true; }
    if number % 2 == 0 { return false; }
    let mut d: u32 = 3;
    while d * d <= number as u32 {
        if number % d as u16 == 0 { return false; }
        d += 2;
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut next: u16 = x;
    loop {
        if next != std::u16::MAX {
            next += 1;
            if is_prime(next) == true {
                return Some(next);
            }
        } else {
            return None;
        }
    }
}

pub fn main() {
    let mut x: u16 = 0;
    while let Some(prime) = next_prime(x) {
        println!("{}", prime);
        x = prime;
    }
    // loop {
    //     let r: Option<u16> = next_prime(x);
    //     match r {
    //         Some(prime) => {
    //             println!("{}", prime);
    //             x = prime;
    //         },
    //         None => {break},
    //     }
    // }
}
