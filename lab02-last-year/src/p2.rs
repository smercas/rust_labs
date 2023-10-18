fn add(x: [u32; 10], y: [u32; 10]) -> [u32; 10] {
    let mut result: [u32; 10] = [0u32; 10];
    let mut i: usize = 0;
    while i < 10 {
        result[i] = x[i] + y[i];
        i += 1;
    }
    result
}

fn mul(x: [u32; 10], y: [u32; 10]) -> [u32; 10] {
    let mut result: [u32; 10] = [0u32; 10];
    let mut i: usize = 0;
    while i < 10 {
        result[i] = x[i] * y[i];
        i += 1;
    }
    result
}

fn print(x: [u32; 10]) {
    let mut i: usize = 0;
    while i < x.len() {
        print!("{} ", x[i]);
        i += 1;
    }
    println!();
}

pub fn main() {
    let x: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let y: [u32; 10] = [101, 102, 103, 104, 105, 106, 107, 108, 109, 110];
    let z: [u32; 10] = add(x, y);
    print(z);
    let a: [u32; 10] = mul(x, z);
    print(a);
}
