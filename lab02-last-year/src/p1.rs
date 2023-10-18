fn is_prime(x: u64) -> bool {
    if x < 2 {
        false
    } else if x == 2 {
        true
    } else if x % 2 == 0 {
        false
    } else {
        let mut d: u64 = 3;
        while d * d <= x {
            if x % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }
}
fn calculate_primes(slice: &mut [u64]) {
    let mut idx: usize = 0;
    let mut next: u64 = 0;
    while idx < slice.len() {
        if is_prime(next) {
            slice[idx] = next;
            idx += 1;
        }
        next += 1;
    }
}
pub fn main() {
    let mut arr: [u64; 10] = [0u64; 10]; // array of 10 elements initialized all with 0
    calculate_primes(&mut arr);

    let mut i: usize = 0;
    while i < arr.len() {
        print!("{} ", arr[i]);
        i += 1;
    }
    println!();
}
