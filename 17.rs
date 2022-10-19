// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen)
// contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

fn number_of_letters_in_written_number (number: u32) -> u32 {
    match number {
        0 => 0, // when calculating 340, it'll be number_of...(300) + number_of...(40) + number_of...(0)
        1 => 3,
        2 => 3,
        3 => 5,
        4 => 4,
        5 => 4,
        6 => 3,
        7 => 5,
        8 => 5,
        9 => 4,
        10 => 3,
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 => 7,
        17 => 9,
        18 => 8,
        19 => 8,
        20 => 6,
        30 => 6,
        40 => 5,
        50 => 5,
        60 => 5,
        70 => 7,
        80 => 6,
        90 => 6,
        100 => 10,
        200 => 10,
        300 => 12,
        400 => 11,
        500 => 11,
        600 => 10,
        700 => 12,
        800 => 12,
        900 => 11,
        1000 => 11,
        _ => {
            let mut ones = number%10;
            let mut tens = ((number/10)%10)*10;
            let hundreds = (number/100)*100;
            if tens == 10 {
                // The case of X hundred and 15 for example, requires us to process the last two digits as one number
                tens = number - hundreds;
                ones = 0;
            }

            if hundreds == 0 {
                // Number is below 100, just need the number of letters in Xty-Y
                return number_of_letters_in_written_number(tens) + number_of_letters_in_written_number(ones);
            } else {
                // Add 3 for 'and', which comes after e.g. three hundred
                return number_of_letters_in_written_number(hundreds) + 3 + number_of_letters_in_written_number(tens) + number_of_letters_in_written_number(ones);
            }
        }
    }
}

fn main () {
    let mut sum = 0;
    for i in 1..=1000 {
        let num_letters = number_of_letters_in_written_number(i);
        sum += num_letters;
    }
    println!("{}", sum);
}
