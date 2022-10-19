// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
// Which starting number, under one million, produces the longest chain?
// NOTE: Once the chain starts the terms are allowed to go above one million.

fn get_next_number_in_collatz_chain (n: u64) -> u64 {
    return if n%2 == 0 { n/2 } else { 3*n + 1 };
}

fn get_collatz_chain_length_starting_at (start: u64) -> u64 {
    let next_number_in_collatz_chain = get_next_number_in_collatz_chain(start);
    if next_number_in_collatz_chain == 1 {
        return 2;
    } else {
        return 1 + get_collatz_chain_length_starting_at(next_number_in_collatz_chain);
    }
}

fn main () {
    let mut n = 1000;
    let mut longest_chain_length = 0;
    let mut longest_chain_starting_number = 1000;
    while n < 1000000 {
        let collatz_chain_length = get_collatz_chain_length_starting_at(n);
        if collatz_chain_length > longest_chain_length {
            longest_chain_length = collatz_chain_length;
            longest_chain_starting_number = n;
        }
        n += 1;
    }
    println!("the longest chain starts at {} and is {} terms long", longest_chain_starting_number, longest_chain_length);
}
