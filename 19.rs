// You are given the following information, but you may prefer to do some research for yourself.

// 1 Jan 1900 was a Monday.
// Thirty days has September,
// April, June and November.
// All the rest have thirty-one,
// Saving February alone,
// Which has twenty-eight, rain or shine.
// And on leap years, twenty-nine.
// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

// 1 Jan 1901 = Tuesday

fn days_in_month (month_index: u16, year: u16) -> u16 {
    match month_index {
        3 | 5 | 8 | 10 => 30,
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        1 => if year%4 == 0 { 29 } else { 28 },
        _ => 0
    }
}

fn day_of_week_index_after_x_days (initial_dow: u16, x_days: u16) -> u16 {
    (initial_dow + x_days)%7
}

// 0 = sunday, 1 = monday, 2 = tuesday....

fn main () {
    let number_of_months_in_century = 12*100;
    let mut current_day_of_week = 2;
    let mut number_of_sunday_first_days_of_month = 0;
    for months_elapsed in 0..number_of_months_in_century {
        let month_index = months_elapsed%12;
        let year = 1901 + months_elapsed/12;
        let days_til_next_start_of_month = days_in_month(month_index, year);
        current_day_of_week = day_of_week_index_after_x_days(current_day_of_week, days_til_next_start_of_month);
        if current_day_of_week == 0 {
            number_of_sunday_first_days_of_month += 1;
        }
    }
    println!("{}", number_of_sunday_first_days_of_month);
}
