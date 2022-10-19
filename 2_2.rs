// Ran in 1.62s or 0.97s
fn main() {
    let mut fib_n1 = 1;
    let mut fib_n2 = 1;
    let mut total = 0;
    loop {
        if fib_n2 > 4000000 {
            break;
        }
        if fib_n2 % 2 == 0 {
            total += fib_n2;
        }
        let new_fib_n1 = fib_n2;
        fib_n2 += fib_n1;
        fib_n1 = new_fib_n1;
    }
    println!("{}", total);
}
