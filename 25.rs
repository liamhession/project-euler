// 9:25 - 9:36  .... 9:37 - 10:02, 10:08 - 10:43
// 11:02 - 11:52
// The Fibonacci sequence is defined by the recurrence relation:

// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:

// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.

// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
mod project_euler_types;
pub use crate::project_euler_types::LongNumber;

fn main () {
  let mut fib_n_2 = LongNumber::new_from_integer(1);
  let mut fib_n_1 = LongNumber::new_from_integer(1);
  let mut fib_n = &fib_n_2 + &fib_n_1;
  let mut n = 3;
  loop {
    fib_n_2 = fib_n_1.clone();
    fib_n_1 = fib_n.clone();
    fib_n = &fib_n_2 + &fib_n_1;
    n += 1;
    if fib_n.num_of_digits() > 999 {
      break;
    }
  }

  println!("{}", fib_n);
  println!("{}", n);
}
