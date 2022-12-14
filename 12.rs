// The sequence of triangle numbers is generated by adding the natural numbers.
//  So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
//  The first ten terms would be:
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
// Let us list the factors of the first seven triangle numbers:
//  1: 1
//  3: 1,3
//  6: 1,2,3,6
// 10: 1,2,5,10
// 15: 1,3,5,15
// 21: 1,3,7,21
// 28: 1,2,4,7,14,28
// We can see that 28 is the first triangle number to have over five divisors.

// What is the value of the first triangle number to have over five hundred divisors?

fn make_triangle_number (n: u32) -> u32 {
    let mut i = 1;
    let mut total = 0;
    while i <= n {
        total += i;
        i += 1;
    }

    total
}

fn divisors_of_number (n: u32) -> Vec<u32> {
    let mut divisors = vec![];
    // Only need to search til the square root is hit, everything after that will already be matched with a previous divisor
    let square_root = ((n as f64).sqrt()).floor() as u32;
    let mut divisor_candidate = 1;
    while divisor_candidate <= square_root {
        let other_divisor = n/divisor_candidate;   // will always be an integer,
        if other_divisor*divisor_candidate == n {  // so it'll only sometimes be an actual, exact divisor
            divisors.push(divisor_candidate);
            if other_divisor != divisor_candidate {
                divisors.push(other_divisor);  // don't push on square roots twice
            }
        }

        divisor_candidate += 1;
    }
    
    divisors
}

fn main () {
    let mut n = 7;
    let mut max_divisors = 0;
    
    
    while max_divisors <= 500 {
        n += 1;

        let triangle_number_n = make_triangle_number(n);
        let divisors = divisors_of_number(triangle_number_n);
        println!("{:?}", divisors);
        
        let divisors_length = divisors.len();
        if divisors_length > max_divisors {
            max_divisors = divisors_length;
        }
    }
    
    println!("{}", n);
    println!("{}", make_triangle_number(n));
}
