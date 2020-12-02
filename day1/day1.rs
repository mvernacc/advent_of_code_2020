use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


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

    let mut success: bool = false;
    let numbers_slice = &numbers[..];
    for x in numbers_slice {
        let needed_value: u32 = 2020 - x;
        success = is_value_in_sorted_slice(needed_value, numbers_slice);
        if success {
            println!("{} * {} = {}", x, needed_value, x * needed_value);
            break;
        }
    }
    if !success {
        println!("No pair of numbers which adds to 2020 was found :(")
    }
}

fn is_value_in_sorted_slice(value: u32, slice: &[u32]) -> bool {
    if slice.is_empty() {
        return false;
    }
    if slice.len() == 1 {
        return value == slice[0];
    }

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