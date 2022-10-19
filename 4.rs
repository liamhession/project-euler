// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
// Ran in 3.12s
fn is_palindrome_number(n: i32) -> bool {
    let number_as_string = n.to_string();
    let half_number_length = number_as_string.len()/2;

    number_as_string.bytes().take(half_number_length).eq(number_as_string.bytes().rev().take(half_number_length))
}

fn main() {
    let top_three_digit_number = 999;
    let mut three_digit_number_one = top_three_digit_number;
    let mut three_digit_number_two = top_three_digit_number;
    let mut biggest_product = 0;
    loop {
        let product = three_digit_number_one * three_digit_number_two;
        if is_palindrome_number(product) {
            println!("{} is a large palindrome made from 2 3-digit numbers: {} x {}", product, three_digit_number_one, three_digit_number_two);
            if product > biggest_product {
                biggest_product = product;
            }
        }
        
        if three_digit_number_one == three_digit_number_two {
            three_digit_number_two -= 1;
            if three_digit_number_two < 100 { break }
            three_digit_number_one = top_three_digit_number;
        } else {
            three_digit_number_one -= 1;
        }
    }
    println!("{} is the biggest product found", biggest_product);
}
