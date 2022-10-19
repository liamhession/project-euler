// FOR SOME REASON THIS DOESN'T WORK AS WRITTEN AND I FEEL LIKE IT REALLY SHOULD...
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which a2 + b2 = c2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.

// If a < b < c, and a+b+c=1000 then the ends of their ranges are a in 1..=332, b in 2..=333, c in 3..=334
fn main() {
    for a in 1..=332 {
        for b in (a+1)..=333 {
            let c = 1000 - a - b;
            let diff = (a*a + b*b) - c*c;
            // if diff > -1000 && diff < 1000 {
            //     println!("pretty close");
            //     println!("{} =/= {}", (a*a + b*b), c*c);
            // }
            if diff == 0 {
                println!("{}", (a*b*c));
            }
        }
    }
}
