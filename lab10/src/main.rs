use std::borrow::Borrow;

fn is_prime(number: &isize) -> bool {
    if *number < 2 { return false; }
    if *number == 2 { return true; }
    if *number % 2 == 0 { return false }
    let mut d: usize = 3;
    while d * d <= *number as usize {
        if *number as usize % d == 0 {
            return false;
        } d = d + 2;
    }
    true
}

mod cache {
    use std::cell::RefCell;
    use std::fmt::Debug;
    use circular_buffer::*;
    pub struct Cache<T: Sized + PartialEq + Clone + Debug, const COUNT: usize> {
        memory_space: RefCell<CircularBuffer<COUNT, (T, bool)>>,
    }
    impl<T: PartialEq + Clone + Debug, const COUNT: usize> Cache<T, COUNT> {
        pub fn new() -> Self {
            Self { memory_space: RefCell::new(CircularBuffer::new()) }
        }
        fn get_from_buffer(&self, elem: &T) -> Option<bool> {
            match self.memory_space.borrow().iter().find(|x| (**x).0 == *elem ) {
                Some((_, result)) => { Some(result.clone()) }
                None => { None }
            }
        }
        pub fn handle_elem(&self, elem: &T, handler: fn(bool, &T), checker: fn(&T) -> bool) {
            #[cfg(debug_assertions)] println!("cache: {:?}", self.memory_space.borrow().to_vec());
            match self.get_from_buffer(elem) {
                Some(result) => {
                    #[cfg(debug_assertions)] println!("cache hit");
                    handler(result, elem);
                },
                None => {
                    #[cfg(debug_assertions)] println!("cache miss");
                    let result = checker(elem);
                    self.memory_space.borrow_mut().push_back((elem.clone(), result));
                    handler(result, elem);
                }
            }
        }
    }
}

#[macro_use] extern crate scan_rules;

fn main() {
    let cache: cache::Cache<isize, 10> = cache::Cache::new();
    let mut exit: bool = false;
    while exit == false {
        readln! (
            (let number: isize) => {
                cache.handle_elem(number.borrow(),
                                  |r, n| {
                                      match r {
                                          true => {
                                              println!("{} is a prime number", n);
                                          },
                                          false => {
                                              println!("{} is not a prime number", n);
                                          }
                                      }
                                  },
                                  is_prime);
            },
            ("quit") => {
                exit = true;
            },
            (..other) => {
                println!("unrecognised input: {}", other);
            }
        );
    }
}
