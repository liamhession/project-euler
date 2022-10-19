// Using names.txt (file containing over five-thousand first names), begin by sorting it into alphabetical order.
// Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
// What is the total of all the name scores in the file?

use std::fs;

fn alphabetize_names (string_of_names: &str) -> Vec<&str> {
  let names_without_prefix = string_of_names.strip_prefix("\"").unwrap();
  let names_without_suffix: &str  = names_without_prefix.strip_suffix("\"").unwrap();
  let mut names_list: Vec<&str> = names_without_suffix.split("\",\"").collect();
  names_list.sort();
  names_list
}

fn alphabetical_value_of_name (name: &str) -> u32 {
  const CAPITAL_A_OFFSET: u32 = 64; // the char value of A is 65, should be normalized to 1
  let mut total = 0;
  for letter in name.chars() {
    let numeric_value_of_char = letter as u32 - CAPITAL_A_OFFSET;
    total += numeric_value_of_char;
  }
  total
}

fn main () {
  let all_names = fs::read_to_string("p022_names.txt").expect("Something went wrong reading the file.");
  let alphabetized_name_list = alphabetize_names(&all_names);

  let mut name_index = 1;
  let mut total = 0;
  for name in alphabetized_name_list {
    let value = alphabetical_value_of_name(&name);
    let name_score = value * name_index;
    total += name_score;
    name_index += 1;
  }

  println!("Total of all name scores: {}", total);
}