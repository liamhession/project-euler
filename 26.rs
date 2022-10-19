// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

// 1/2	= 	0.5
// 1/3	= 	0.(3)
// 1/4	= 	0.25
// 1/5	= 	0.2
// 1/6	= 	0.1(6)
// 1/7	= 	0.(142857)
// 1/8	= 	0.125
// 1/9	= 	0.(1)
// 1/10	= 	0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
mod project_euler_types;
pub use crate::project_euler_types::LongNumber;

fn main () {
  let mut longest_recurring_cycle = 6;
  let mut d_with_longest_cycle = 7;

  for d in 11..19 {
    let decimal = (10000000000000000000000000000000000_u128)/(d as u128);
    let decimal_digits = decimal.to_string();

    // Need to find a starting point and length which if used to divide the string covers everything
    let mut start_repeat_index = 0;
    let mut repeat_length = 1;
    loop {
      let repeating_portion = &decimal_digits[start_repeat_index..(start_repeat_index+repeat_length)];
      println!("{:?}", repeating_portion);

      let can_cover_string = true; //decimal_digits[0..start_repeat_index];
      if can_cover_string {
        break;
      }
      repeat_length += 1;
    }
  }
}