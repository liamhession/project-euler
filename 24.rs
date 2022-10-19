
// A permutation is an ordered arrangement of objects.
//    For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
//    If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
//    The lexicographic permutations of 0, 1 and 2 are:
//    012   021   102   120   201   210
//    for 0, 1, 2, 3:
//    0123  0132  0213  0231  0312  0321  1023  1032  1203  1230  1302  1320  ...
//       n-1   n-2   n-1   n-2   n-1   n-3   n-1   n-2   n-1   n-2   n-1
//    Algorithm, starting with them in order 012345:
//      grow_permutation_at_place_p(n-1) 0123 5 < > 4
//      is it greater than previous? yes
//      grow at n-2              012435
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn step_through_permutations_lexographically(digits: &mut [u8]) {
  // We'll know we've gotten to the highest-value permutation when it equals the reverse of the input
  let final_permutation: Vec<u8> = digits.iter().rev().map(|digit| *digit).collect(); // ONLINE: Is there a more efficient way?
  let mut digits_arent_in_final_permutation = true;

  let permutation_we_want_to_see = 1000000;
  let mut current_permutation = 1;

  let length = digits.len();
  // Start by trying to grow the second-to-last digit, always escalate to closer-to-start digit when growing fails
  while digits_arent_in_final_permutation {
    if current_permutation == permutation_we_want_to_see {
      println!("{:?}", digits);
    }
    current_permutation += 1;

    let mut digit_to_change = length - 2;
    loop {
      let successfully_grew = grow_permutation_at_place_p(digits, digit_to_change);
      if successfully_grew { break }
      digit_to_change -= 1;
    }

    // Assume this got us to the final permutation, then disprove it if any digits differ between the two
    //    (could be more succinct as a reduce statement that sees if any of them are diff)
    digits_arent_in_final_permutation = false;
    for (index, digit) in digits.iter().enumerate() {
      if final_permutation[index] != *digit {
        digits_arent_in_final_permutation = true;
      }
    }
  }
}

fn grow_permutation_at_place_p(digits: &mut [u8], p: usize) -> bool {
  let current_digit_at_p = digits[p];
  let mut next_highest_digit_after_p: u8 = 10;
  let mut index_of_next_highest_digit = p;

  let remaining_length = digits.len();
  for (index, digit) in digits[p+1..remaining_length].iter().enumerate() {
    if *digit > current_digit_at_p && *digit < next_highest_digit_after_p {
      next_highest_digit_after_p = *digit;
      index_of_next_highest_digit = p+1+index;
    }
  }

  // No digit after p is larger, and could replace it, so we aren't going to grow the permutation at this level
  if next_highest_digit_after_p == 10 && index_of_next_highest_digit == p {
    return false;
  }

  digits[p] = next_highest_digit_after_p;
  digits[index_of_next_highest_digit] = current_digit_at_p;

  rearrange_to_smallest_permutation(&mut digits[p+1..remaining_length]);
  true
}

fn rearrange_to_smallest_permutation(digits: &mut [u8]) {
  if digits.len() == 1 {
    return;
  }

  else if digits.len() == 2 {
    if digits[0] > digits[1] {  // [3, 1] for example
      let holder = digits[1];
      digits[1] = digits[0];
      digits[0] = holder;
    }
  }
  
  else {
    // Will put the smallest available digit in the first position, recursively call fn on the rest
    let mut position_of_smallest_digit = 0;
    let mut smallest_digit = 10;
    for (index, digit) in digits.iter().enumerate() {
      if *digit < smallest_digit {
        smallest_digit = *digit;
        position_of_smallest_digit = index;
      }
    }

    digits[position_of_smallest_digit] = digits[0];
    digits[0] = smallest_digit;
    
    let remaining_length = digits.len();
    rearrange_to_smallest_permutation(&mut digits[1..remaining_length]);
  }
}

fn main () {
  // let digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  step_through_permutations_lexographically(&mut digits);
}

// more from 9:28 - 9:52
// 5:14 - 6:09
//
//ANSWER TO SUBMIT: 2783915460