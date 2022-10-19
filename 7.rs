// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn is_prime(n: u32) -> bool {
    let half = n/2;
    for i in 2..=half {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn prime_number_x(x: u32) -> u32 {
    let mut primes_found = 0;
    let mut current_number = 1;
    
    while primes_found < x {
        current_number += 1;
        if is_prime(current_number) {
            primes_found += 1;
        }
    }

    current_number
}

fn main() {
    let x = 10001;
    println!("{} is prime number {}", prime_number_x(x), x);
}
