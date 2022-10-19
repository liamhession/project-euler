// 10:45 - 11:05

// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?
// 2^1000 = 2^500*2^500 = 2^20*2^20*2^20... 50 times
// Using u128, could store ten 2^100s but then need to multiply them by hand to create 2^1000
// It could be a good exercise to create a struct that allows for huge numbers to be stored as string, implement multiplication

// Define a new struct that will store long numbers and allow multiplication
use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::cmp;

#[derive(Debug)]
struct LongNumber {
    digits: Vec<u8>,
}

impl LongNumber {
    fn new(digits: String) -> Self {
        // Store the digits of the string from 1s place to 10s place, up to highest place
        let digits_array: Vec<u8> = digits.chars().rev().map(|digit| digit.to_digit(10).unwrap() as u8).collect();
        Self {
            digits: digits_array,
        }
    }

    fn new_from_digits(digits: Vec<u8>) -> Self {
        Self {
            digits: digits,
        }
    }
}

impl fmt::Display for LongNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reversed_digits_string: String = self.digits.iter().rev().map(|digit| std::char::from_digit(*digit as u32, 10).unwrap()).collect();
        write!(f, "{}", reversed_digits_string)
    }
}

impl Add for LongNumber {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        let lhs_array = self.digits;
        let rhs_array = rhs.digits;
        let lhs_length = lhs_array.len();
        let rhs_length = rhs_array.len();

        let result_max_size = cmp::max(lhs_length, rhs_length) + 1;
        let mut result_array = Vec::<u8>::new();
        
        let mut carryover = 0;
        for i in 0..result_max_size {
            let lhs_digit = if i >= lhs_length { 0 } else { lhs_array[i] };
            let rhs_digit = if i >= rhs_length { 0 } else { rhs_array[i] };
            let sum = carryover + lhs_digit + rhs_digit;
            
            if sum > 9 {
                carryover = 1;
                let result_digit = sum - 10;
                result_array.push(result_digit);
            } else {
                carryover = 0;
                result_array.push(sum);
            }
        }
        if carryover == 1 {
            result_array.push(1);
        }

        Self {
            digits: result_array,
        }
    }
}

impl Mul for LongNumber {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self {
        let lhs_array = self.digits;
        let rhs_array = rhs.digits;
        let lhs_length = lhs_array.len();
        let rhs_length = rhs_array.len();
        
        let (top_number, bottom_number) = if lhs_length > rhs_length { (lhs_array, rhs_array) } else { (rhs_array, lhs_array) };
        // TODO: all below this not yet tested
        let mut array_of_products_to_sum = Vec::<LongNumber>::new();
        for i in 0..bottom_number.len() {
            let bottom_digit_to_multiply = bottom_number[i];
            let mut product_array = Vec::<u8>::new();

            let mut carryover = 0;
            for j in 0..top_number.len() {
                let top_digit_to_multiply = top_number[j];
                let product = bottom_digit_to_multiply*top_digit_to_multiply + carryover;

                if product > 9 {
                    carryover = product/10;
                    let result_digit = product % 10;
                    product_array.push(result_digit);
                } else {
                    carryover = 0;
                    product_array.push(product);
                }
            }
            if carryover > 0 {
                product_array.push(carryover);
            }
            
            // Add i "trailing" 0s, though in the case of our struct they're leading 0s
            for _x in 0..i {
                product_array.insert(0, 0);
            }
            
            let product_to_add = LongNumber::new_from_digits(product_array);
            array_of_products_to_sum.push(product_to_add);
        }
        
        let mut final_product = LongNumber::new("0".to_string());
        for addend in array_of_products_to_sum.into_iter() {
            final_product = final_product + addend;
        }
        
        final_product
    }
}

fn main () {
    let two: i128 = 2;
    let two_to_the_hundred_number = two.pow(100);
    let y1 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y2 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y3 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y4 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y5 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y6 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y7 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y8 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y9 = LongNumber::new(two_to_the_hundred_number.to_string());
    let y10 = LongNumber::new(two_to_the_hundred_number.to_string());
    let z = y1 * y2 * y3 * y4 * y5 * y6 * y7 * y8 * y9 * y10;
    println!("{}", z);
}
