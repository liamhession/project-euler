// Problem 51 - first one w difficulty above 5% (15%)
// By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values:
//     13, 23, 43, 53, 73, and 83, are all prime.
// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers,
//     yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
//     Consequently 56003, being the first member of this family, is the smallest prime with this property.
// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.
fn make_two_digit_prime_family (digit_one: Option<u8>, digit_two: Option<u8>) -> Vec<u8> {
    let all_options_with_first_digit_set = match digit_one {
        Some(a) => {
            let mut options = vec![];
            for b in 0..=9 {
                let two_digit_number = format!("{}{}", a, b).parse::<u8>().unwrap();
                options.push(two_digit_number);
            }
            options
        },
        None => vec![],
    };
    
    all_options_with_first_digit_set
}

fn main () {
    let two_digit_family = make_two_digit_prime_family(Some(3), None);
    println!("{:?}", two_digit_family);
}
