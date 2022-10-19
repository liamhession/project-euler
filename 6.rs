// The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn sum_of_the_squares(max: i32) -> i32 {
    let mut sum = 0;
    for n in 1..=max {
        sum += n.pow(2);
    }
    sum
}

fn square_of_the_sum(max: i32) -> i32 {
    let sum: i32 = (1..=max).sum();
    sum.pow(2)
}

fn difference(a: i32, b: i32) -> i32 {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

fn main() {
    let max = 100;
    let sotsq = sum_of_the_squares(max);
    let sqots = square_of_the_sum(max);
    let diff = difference(sotsq, sqots);

    println!("difference between sum of the squares and square of the sums for 1 to {} = {}", max, diff);
}
