//! Base 64 encoder and decoder library

fn alphabet(input: u8) -> char {
    match input {
        0..=25 => (input + ('A' as u8)) as char,
        26..=51 => (input - 26 + ('a' as u8)) as char,
        52..=61 => (input - 52 + ('0' as u8)) as char,
        62 => '+',
        63 => '/',
        _ => unreachable!(),
    }
}
/// function used to encode an ascii message into base 64
/// # Example
/// ```
/// # use base64::encode;
/// assert_eq!(encode("Many hands make light work.".as_bytes()), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
/// assert_eq!(encode("Man".as_bytes()), "TWFu");
/// assert_eq!(encode("Ma".as_bytes()), "TWE=");
/// assert_eq!(encode("M".as_bytes()), "TQ==");
/// assert_eq!(encode("".as_bytes()), "");
/// ```
pub fn encode(input: &[u8]) -> String {
    let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
    for idx in 0..(input.len() / 3) {
        let (e1, e2, e3) = (input[idx * 3], input[idx * 3 + 1], input[idx * 3 + 2]);
        result.push(alphabet(e1 >> 2));
        result.push(alphabet(((e1 & 0b_0000_0011) << 4) | (e2 >> 4)));
        result.push(alphabet(((e2 & 0b_0000_1111) << 2) | (e3 >> 6)));
        result.push(alphabet(e3 & 0b_0011_1111));
    }
    let idx = input.len() - input.len() % 3;
    if input.len() % 3 >= 1 {
        result.push(alphabet(input[idx] >> 2));
        if input.len() % 3 == 1 {
            result.push(alphabet((input[idx] & 0b_0000_0011) << 4));
            result.push('=');
        } else {
            result.push(alphabet(((input[idx] & 0b_0000_0011) << 4) | input[idx + 1] >> 4));
            result.push(alphabet((input[idx + 1] & 0b_0000_1111) << 2));
        }
        result.push('=');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn many_hands() {
        assert_eq!(encode("Many hands make light work.".as_bytes()), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
    }
    #[test]
    fn empty_string() {
        assert_eq!(encode("".as_bytes()), "");
    }
    #[test]
    fn light_work() {
        assert_eq!(encode("light work.".as_bytes()), "bGlnaHQgd29yay4=");
        assert_eq!(encode("light work".as_bytes()), "bGlnaHQgd29yaw==");
        assert_eq!(encode("light wor".as_bytes()), "bGlnaHQgd29y");
        assert_eq!(encode("light wo".as_bytes()), "bGlnaHQgd28=");
        assert_eq!(encode("light w".as_bytes()), "bGlnaHQgdw==");
        assert_eq!(encode("light ".as_bytes()), "bGlnaHQg");
    }
}
