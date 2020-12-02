use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Run with `./day1 input.txt`
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{}", filename);

    let mut numbers = Vec::<u32>::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(x_str) = line {
                let x: u32 = x_str.parse().unwrap();
                numbers.push(x);
            }
        }
    }
    numbers.sort();
    println!("{:?}", numbers);

    let numbers_slice = &numbers[..];
    // Solve part 
    // This approach runs in O(n log(n)) time.
    println!("Part 1:");
    let pair = find_two_values_sum(2020, numbers_slice);
    if pair.0 == 0 && pair.1 == 0 {
        println!("No pair of numbers which adds to 2020 was found :(");
    } else {
        println!("{} * {} = {}", pair.0, pair.1, pair.0 * pair.1);
    }

    // Solve part 2
    // This approach re-uses the `find_two_values_sum` function from part 1.
    // It runs in O(n^2 log(n)) time.
    println!("Part 2:");
    let mut success = false;
    for (i, x) in numbers_slice.iter().enumerate() {
        // We need to find two other numbers in the input list which add up to `2020 - x`.
        let needed_sum: u32 = 2020 - x;
        let numbers_without_x = [&numbers_slice[..i], &numbers_slice[i+1..]].concat();
        let pair = find_two_values_sum(needed_sum, &numbers_without_x[..]);
        if !(pair.0 == 0 && pair.1 == 0) {
            success = true;
            println!("{} * {} * {} = {}", x, pair.0, pair.1, x * pair.0 * pair.1);
            break;
        }
    }
    if !success {
        println!("No triplet of numbers which adds to 2020 was found :(")
    }

}

/// Find two values in a sorted slice which add up to `sum`.
/// Returns (0, 0) if no valid pair is found.
fn find_two_values_sum(sum: u32, sorted_slice: &[u32]) -> (u32, u32) {
    for x in sorted_slice {
        if *x > sum { continue; }
        let needed_value: u32 = sum - x;
        if is_value_in_sorted_slice(needed_value, sorted_slice) {
            return (*x, needed_value);
        }
    }
    return (0, 0);
}

/// Check if a value is present in a sorted slice.
fn is_value_in_sorted_slice(value: u32, slice: &[u32]) -> bool {
    // Base cases.
    if slice.is_empty() {
        return false;
    }
    if slice.len() == 1 {
        return value == slice[0];
    }

    // Do recursive binary search.
    let split_index: usize = slice.len() / 2;
    let split_value: u32 = slice[split_index];
    if split_value == value {
        return true;
    } else if split_value < value {
        return is_value_in_sorted_slice(value, &slice[split_index + 1..]);
    }
    else {
        return is_value_in_sorted_slice(value, &slice[..split_index]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
