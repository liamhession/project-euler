// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28
// would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24.
// By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However,
// this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed
// as the sum of two abundant numbers is less than this limit.
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

fn get_divisors (i: u32) -> Vec<u32> {
  let mut divisors = vec![1];
  let upper_limit = f32::sqrt(i as f32) as u32 + 1;
  for d in 2..(upper_limit) {
    if !divisors.contains(&d) && i%d == 0 {
      let a = i/d;
      divisors.push(d);
      if a != d {
        divisors.push(a);
      }
    }
  }

  divisors
}

fn this_is_an_abundant_number (i: u32) -> bool {
  let divisors = get_divisors(i);
  divisors.iter().sum::<u32>() > i
}

fn this_can_be_written_as_the_sum_of_two_abundant_numbers(i: u32, abundant_numbers: &Vec<u32>) -> bool {
  for a in abundant_numbers.iter() {
    let b = i - a;
    if abundant_numbers.contains(&b) {
      // println!("{} can be written as a sum of abundants {} + {}", i, b, a);
      return true;
    }
  }

  false
}

fn main () {
  // They tell us 24 is the smallest, in the intro
  const LOWEST_NUMBER_KNOWN_TO_BE_SUM_OF_TWO_ABUNDANT_NUMBERS: u32 = 28124;

  let mut positive_integers_that_cant_be_written_as_sum_of_two_abundant_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];
  let mut abundant_numbers = vec![12];

  for i in 25..LOWEST_NUMBER_KNOWN_TO_BE_SUM_OF_TWO_ABUNDANT_NUMBERS {
    if this_is_an_abundant_number(i) {
      // println!("abundant: {}", i);
      abundant_numbers.push(i);
    }
    if !this_can_be_written_as_the_sum_of_two_abundant_numbers(i, &abundant_numbers) {
      positive_integers_that_cant_be_written_as_sum_of_two_abundant_numbers.push(i);
    }
  }

  println!("{}", positive_integers_that_cant_be_written_as_sum_of_two_abundant_numbers.iter().sum::<u32>());
}
