// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
// Evaluate the sum of all the amicable numbers under 10000.

fn sum_of_proper_divisors (num: u32) -> u32 {
  let mut divisors = vec![1]; // 1 is always a divisor
  let halfway = num/2;
  for i in 2..=halfway {
    let potential_divisor = num/i;
    if potential_divisor*i == num {
      divisors.push(i);  // improvement: add both, do check for ones already added
    }
  }

  divisors.iter().sum()
}

fn main () {
  let mut amicable_pairs = Vec::new();
  for a in 1..10000 {
    let sopd_a = sum_of_proper_divisors(a);
    let sopd_b = sum_of_proper_divisors(sopd_a);
    if a != sopd_a && sopd_b == a {
      if !amicable_pairs.contains(&a) {
        amicable_pairs.push(a);
        amicable_pairs.push(sopd_a);
      }
    } 
  }
  let sum_amicable_pairs: u32 = amicable_pairs.iter().sum();
  println!("{}", sum_amicable_pairs);
}