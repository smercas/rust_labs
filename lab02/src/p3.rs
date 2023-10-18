pub fn add_space(s: &mut String, n: usize) {
    s.reserve(n);
    let mut i: usize = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}

pub fn add_str(s: &mut String, to_add: &str) {
    s.push_str(&to_add);
}

pub fn add_integer(s: &mut String, number: i32) {
    if number == 0 {
        s.push('0');
    } else {
        let mut size: usize = 0;
        let mut underscore_counter: usize;
        let mut copy: i32 = number.abs();
        while copy > 0 {
            size += 1;
            copy = copy / 10;
        }
        underscore_counter = size;
        size += (size - 1) / 3;
        if number < 0 {
            size += 1;
        }
        s.reserve(size);
        if number < 0 {
            s.push('-');
        }
        size = s.len();
        copy = number.abs();
        while copy > 0 {
            s.insert(size, ((copy % 10) as u8 + '0' as u8) as char);
            copy = copy / 10;
            underscore_counter -= 1;
            if underscore_counter % 3 == 0 && underscore_counter != 0 {
                s.insert(size, '_');
            }
        }
    }
}

pub fn add_float(s: &mut String, number: f32) {
    let mut integral_part: i32 = number.abs() as i32;
    let mut fractional_part: i32 = (number.fract() * 1000f32) as i32;
    if number < 0f32 {
        s.push('-');
    }
    if integral_part == 0 {
        s.push('0');
    } else {
        let original_length: usize = s.len();
        while integral_part > 0 {
            s.insert(original_length, ((integral_part % 10) as u8 + '0' as u8) as char);
            integral_part /= 10;
        }
    }
    s.push('.');
    if fractional_part == 0 {
        s.push_str("000");
    } else {
        let original_length: usize = s.len();
        let mut zero_counter: i32 = 0;
        while fractional_part > 0 {
            s.insert(original_length, ((fractional_part % 10) as u8 + '0' as u8) as char);
            fractional_part /= 10;
            zero_counter += 1;
        }
        while zero_counter < 3 {
            s.insert(original_length, '0');
            zero_counter += 1;
        }
    }
}

pub fn main() {
    let mut s: String = String::new();
    add_space(&mut s, 40);
    s.push_str("I 💚\n");
    add_space(&mut s, 40);
    s.push_str("RUST\n\n");
    add_space(&mut s, 4);
    s.push_str("Most");
    add_space(&mut s, 12);
    s.push_str("crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306_437_968);
    add_space(&mut s, 11);
    s.push_str("and");
    add_space(&mut s, 5);
    s.push_str("lastest");
    add_space(&mut s, 9);
    s.push_str("is\n");
    add_space(&mut s, 9);
    s.push_str("downloaded");
    add_space(&mut s, 8);
    s.push_str("has");
    add_space(&mut s, 13);
    s.push_str("downloads");
    add_space(&mut s, 5);
    s.push_str("the");
    add_space(&mut s, 9);
    s.push_str("version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    s.push('.');
    println!("{s}");
}