// Make a square root function for floating numbers.
// Proposed algorithm: use a binary search and at each iteration try if the squared of that number is close enough to the input.
// Remember that floating point calculations are not precise, and not all numbers can be represented exactly.
// Fun fact: different hardware can give slightly different results to floating operations.
// Because of this, direct equality (==) will most likely not work, so use an epsilon to check if the numbers are close enough.
// const EPSILON: f32 = 0.01;
// How is the square root of a number in the interval (0, 1)?
fn binary_search(mut lower: f32, mut upper: f32, x: &f32) -> f32{
  const EPSILON: f32 = 0.01;
  let mut middle: f32 = (lower + upper) / 2.0;
  let mut square: f32 = middle * middle;
  while (square - *x).abs() >= EPSILON {
#[cfg(feature = "debug_text")] {
    println!("upper: {} | lower: {} | middle: {} | square: {} | (square - *x).abs(): {} | EPSILON: {}", upper, lower, middle, square, (square - *x).abs(), EPSILON);
}
    if square < *x {
      lower = middle;
    } else {
      upper = middle;
    }
    middle = (lower + upper) / 2.0;
    square = middle * middle;
  }
#[cfg(feature = "debug_text")] {
  println!("upper: {} | lower: {} | middle: {} | square: {} | (square - *x).abs(): {} | EPSILON: {}", upper, lower, middle, square, (square - *x).abs(), EPSILON);
}
  middle
}

fn sqrt(x: &f32) -> f32 {
  if *x < 0.0 {
    panic!("Cannot compute the square root of a negative number :(");
  }
  if *x < 1.0 {
    return binary_search(0.0, 1.0, x);
  }
  binary_search(1.0, *x, x)
}

fn print_sqrt(x: &f32) {
  println!("{:>4} -> {:>5.2}", x, sqrt(x));
  // {:>4}   -> this means pad it to the right with spaces to always occupy 4 characters
  // {:>5.2} -> this means pad it to the right with spaces to always occupy 5 characters, and always print the float with 2 decimals
}

pub fn main() {
  print_sqrt(&4.0);
  print_sqrt(&16.0);
  print_sqrt(&2.0); // ~1.41
  print_sqrt(&3.0); // ~1.73
  print_sqrt(&111.0); // ~10.53
  print_sqrt(&0.5); // ~0.7
  print_sqrt(&3.14); // ~1.77
  print_sqrt(&1.0); // ~1 👀
  print_sqrt(&1970.0); // ~44.38
}
