// With memoization this runs near-instantly instead of taking 15 minutes, big win!

// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
//      there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?  8:40
use std::collections::HashMap;

fn get_number_of_remaining_combinations (cache: &mut HashMap<(u8, u8), u64>, number_of_rights: u8, number_of_downs: u8) -> u64 {
    if number_of_rights == 0 || number_of_downs == 0 {
        return 1;
    } else {
        let remaining_moves_key = (number_of_rights, number_of_downs);
        let memoized_result = cache.get(&remaining_moves_key);
        match memoized_result.map(|entry| entry.clone()) { // makes the option's value concrete, so you don't have to dereference result below
            Some(result) => {
                return result;
            },
            None => {
                let result = get_number_of_remaining_combinations(cache, number_of_rights-1, number_of_downs) +
                    get_number_of_remaining_combinations(cache, number_of_rights, number_of_downs-1);
                cache.insert(remaining_moves_key, result.clone());
                return result;
            }
        }
    }
}
fn main () {
    let grid_size = 20;

    let number_of_rights = grid_size;
    let number_of_downs = grid_size;
    
    let mut cache = HashMap::new();

    let combos = get_number_of_remaining_combinations(&mut cache, number_of_rights, number_of_downs);
    println!("{}", combos);
}
