// The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.
// Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

fn product_of_string(numeric_string: Option<&str>) -> u64 {
    match numeric_string {
        Some(num) => {
            let mut product = 1;
            for n in num.chars() {
                let int_n = n.to_digit(10).map(u64::from); // convert to u64 integer in decimal notation

                let digit_to_multiply = match int_n {
                    Some(actual_number) => actual_number,
                    None => {
                        println!("There was a non-digit character in the string, '1' used instead, to not ruin product");
                        1
                    }
                };
                product *= digit_to_multiply;
            }
            return product;
        }
        None => 0
    }
}

fn main() {
    let long_ass_number = "\
73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450";

    let mut max_product_found = 0;
    let final_start_of_13_gram = long_ass_number.chars().count() - 14;
    
        println!("{}", final_start_of_13_gram);
    for index in 0..=final_start_of_13_gram {
        let a_13_gram = long_ass_number.get(index..(index+13));
        let product = product_of_string(a_13_gram);
        
        if product > max_product_found {
            max_product_found = product;
            println!("product of {:?} is", a_13_gram);
            println!("{:?}", product);
        }
    }
}
