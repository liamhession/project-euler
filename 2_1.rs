// Ran in 1.57s or 1.82s
fn fib_n(n: i32) -> i32 {
  if n <= 1 {
    1
  } else {
    fib_n(n-1)+fib_n(n-2)
  }
}

fn main() {
    let mut n = 1;
    let mut total = 0;
    loop {
        let fib = fib_n(n);
        if fib > 4000000 {
            break;
        }
        if fib % 2 == 0 {
            total += fib;
        }
        n += 1;
    }
    println!("{}", total);
}
