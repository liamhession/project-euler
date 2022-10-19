// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.
//              This version ran in around 3 minutes to find the solution for max = 2M
fn is_prime (num: u64, primes_thus_far: &Vec<u64>) -> bool {
    let half = num/2;
    for prime_factor in primes_thus_far.iter() {
        // Had >= originally, which meant when it was 4's turn, it got marked as prime
        if *prime_factor > half {
            break;
        }
        if num % prime_factor == 0 {
            return false;
        }
    }

    true
}
fn primes_below (max: u64) -> Vec<u64> {
    let mut all_under = vec![];
    let mut num = 2;
    loop {
        if num >= max {
            return all_under;
        }
        if is_prime(num, &all_under) {
            all_under.push(num);
        }
        num += 1;
    }
}
fn main () {
    let max = 2000000;
    let primes_up_to_max = primes_below(max);
    let sum: u64 = primes_up_to_max.iter().sum();
    println!("{:?}", sum);
}
