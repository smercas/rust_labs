fn add_chars_n(mut s: String, c: char, n: usize) -> String {
    s.reserve(n);
    let mut i: usize = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
    return s;
}

pub fn main() {
    let mut s: String = String::from("");
    let mut i: usize = 0;
    while i < 26 {
        let c: char = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
}
