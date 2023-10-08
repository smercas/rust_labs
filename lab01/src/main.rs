pub mod p1;
pub mod p2;
pub mod p3;

fn main() {
    p1::prime_numbers(0, 100);
    p2::coprime_numbers(0, 100, 0, 100);
    p3::bottles_song(99);
}
