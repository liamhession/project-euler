// 12:50 - 1:15

fn check_rarity_of_digits_in_powers(base: i64) {
  let digit_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

  let mut first_power_with_digit_x = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  let mut power = 2;

  while first_power_with_digit_x.contains(&0) {
    let exponent = base.pow(power);
    for (i, digit) in digit_chars.iter().enumerate() {
      if exponent.to_string().chars().collect::<Vec<char>>().contains(&digit) {
        if first_power_with_digit_x[i] == 0 {
          println!("{}^{} = {} which has a {} in it", base, power, exponent, digit);
          first_power_with_digit_x[i] = power;
        }
      }
    }
    power += 1;
  }

  println!("Final array of power needed to see each digit, for {}:", base);
  println!("{:?}", first_power_with_digit_x);
}

fn main() {
  for i in 11..20 {
    check_rarity_of_digits_in_powers(i);
  }
}