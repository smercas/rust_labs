fn print_slice(x: &[u64]) {
  let mut i: usize = 0;
  while i < x.len() {
      print!("{} ", x[i]);
      i += 1;
  }
}

fn modify_slice(x: &mut [u64]) {
  let mut i: usize = 0;
  while i < x.len() {
      x[i] += 1;
      i += 1;
  }
}

pub fn main() {
  let mut arr: [u64; 5] = [1, 2, 3, 4, 5];
  /*
  {
    let s1: &mut [u64] = &mut arr[..3];
    modify_slice(s1);
    print_slice(s1);
  }
  {
    let s2: &mut [u64] = &mut arr[3..];
    modify_slice(s2);
    print_slice(s2);
  }
  println!();
  print_slice(&arr);
  println!();
  */
  //can't really think of any other way of doing this with slices
  //maybe an array of mutable references but I've tried that and I have to pass the size at compile time for some reason
  //nvm I found out
  //and it also makes sense
  let (s1, s2) = arr.split_at_mut(3);
  modify_slice(s1);
  modify_slice(s2);
  print_slice(s1);
  print_slice(s2);
  println!();
  print_slice(&arr);
  println!();
}