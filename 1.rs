fn main() {
    let limit = 1000;
    let sum_of_multiples = |n| {
        let m = (limit - 1)/n;
        (n * m * (m+1))/2
    };
    println!("{}", sum_of_multiples(3) + sum_of_multiples(5) - sum_of_multiples(15));
}
