// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//      Got this through brute-force prime factorization, but let's make a gen-purpose program: 232,792,560
use std::collections::HashMap;

fn prime_factors(number: u32) -> HashMap<u32, u32> {
    let mut factors_dict: HashMap<u32, u32> = HashMap::<u32, u32>::new();

    if number > 40 {
        println!("We can't guarantee the prime_factorization of any number greater than 40");
        return factors_dict;
    }

    let relevant_primes = [2, 3, 5, 7, 11, 13, 17, 19];

    let mut remaining_number = number;
    
    while remaining_number > 1 {
        for prime_ref in relevant_primes.iter() {
            let prime = *prime_ref;
            if remaining_number % prime == 0 {
                remaining_number /= prime;

                let current_exponent = factors_dict.entry(prime).or_insert(0);
                *current_exponent += 1;
            }
        }
    }
    
    factors_dict
}

fn main() {
    let max = 20;

    let mut factors_seen: HashMap<u32, u32> = HashMap::<u32, u32>::new();

    for n in 2..=max {
      let prime_factors_for_number = prime_factors(n);
      for (prime, exponent) in prime_factors_for_number.into_iter() {
        // When this prime has not yet shown, insert the current number's required exponent for it
        let current_exponent = factors_seen.entry(prime).or_insert(exponent);
        // When it was already there, it's possible that the current number's required exponent is higher
        if exponent > *current_exponent {
            factors_seen.insert(prime, exponent);
        }
      }
    }
    
    let mut number_divisible_by_all = 1;
    for (prime, exponent) in factors_seen.into_iter() {
        number_divisible_by_all *= u32::pow(prime, exponent);
    }
    
    println!("The smallest number that is evenly divisible by all integers up to {} is {}", max, number_divisible_by_all);
}
