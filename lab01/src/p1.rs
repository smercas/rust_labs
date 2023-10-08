fn is_prime(number: i128) -> bool {
    if number < 2 {
        false
    } else if number == 2 {
        true
    } else if number % 2 == 0 {
        false
    } else {
        let mut d: i128 = 3;
        while d * d <= number {
            if number % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }
}
pub fn prime_numbers(start: i128, finish: i128) {
    let mut found_at_least_one_element: bool = false;
    let mut i: i128 = start;
    while i <= finish {
        if is_prime(i) {
            if found_at_least_one_element {
                print!(", ");
            } else {
                print!("Prime numbers: ");
            }
            print!("{}", i);
            found_at_least_one_element = true;
        }
        i += 1;
    }
    println!();
}
