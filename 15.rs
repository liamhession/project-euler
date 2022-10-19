// Took over 15 minutes to run, and it was a lot faster to just google 40 choose 20 once i figured out that was the pattern

// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
//      there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?

fn get_number_of_remaining_combinations (number_of_rights: i64, number_of_downs: i64) -> i64 {
    if number_of_rights == 0 || number_of_downs == 0 {
        return 1;
    } else {
        return get_number_of_remaining_combinations(number_of_rights-1, number_of_downs) + 
            get_number_of_remaining_combinations(number_of_rights, number_of_downs-1);
    }
}
fn main () {
    let grid_size = 20;
    
    let number_of_rights = grid_size;
    let number_of_downs = grid_size;
    
    let combos = get_number_of_remaining_combinations(number_of_rights, number_of_downs);
    println!("{}", combos);
}
