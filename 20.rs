// n! means n × (n − 1) × ... × 3 × 2 × 1
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// Find the sum of the digits in the number 100!

mod project_euler_types;
pub use crate::project_euler_types::LongNumber;

fn factorial (num: u128) -> LongNumber {
    let mut product = LongNumber::new_from_integer(1);
    for i in 1..=num {
        let multiplicand = LongNumber::new_from_integer(i);
        product = product * multiplicand;
    }
    product
}

fn main () {
    let large_factorial = factorial(100);
    println!("{}", large_factorial);
    
    let sum = large_factorial.sum_of_digits(); 
    println!("sum of digits: {}", sum);
}
