fn coprime(mut a: i128, mut b: i128) -> bool {
    let mut r: i128;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    a == 1
}
pub fn coprime_numbers(first_start: i128, first_finish: i128, second_start: i128, second_finish: i128) {
    let mut found_at_least_one_element: bool = false;
    let mut i: i128;
    let mut j: i128;
    i = first_start;
    while i <= first_finish {
        j = second_start;
        while j <= second_finish {
            if coprime(i, j) {
                if found_at_least_one_element {
                    print!(", ");
                } else {
                    print!("Coprime numbers: ");
                }
                print!("({} {})", i, j);
                found_at_least_one_element = true;
            }
            j += 1;
        }
        i += 1;
    }
    println!();
}
