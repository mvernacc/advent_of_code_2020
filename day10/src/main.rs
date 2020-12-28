use std::{fs, convert::TryFrom};

fn main() {
    println!("Hello, world!");
}

fn compute_jolt_differences(adapters: &[u32]) -> Option<Box<[u32; 4]>> {
    // If `difference_counts[i] == k`, there are `k` joltage differences of `i` jolts
    // in the adapter chain.
    let mut difference_counts = [0, 0, 0, 0];
    let mut unused_adapters = adapters.to_vec();
    // The jolt rating of the built-in joltage adapter.
    let built_in_joltage = adapters.iter().max().unwrap() + 3;
    unused_adapters.push(built_in_joltage);

    unused_adapters.sort();
    unused_adapters.reverse();

    // "The charging outlet has an effective rating of 0 jolts", so start our adapter chain
    // at 0 jolts.
    let mut jolt_level = 0;
    while !unused_adapters.is_empty() {
        let lowest_adapter = unused_adapters.pop().unwrap();
        let difference = lowest_adapter - jolt_level;
        if difference < 1 {
            println!("Lowest adapter {} too low for jolt level {}.", lowest_adapter, jolt_level);
            return None;
        }
        if difference > 3 {
            println!("Lowest adapter {} too high for jolt level {}.", lowest_adapter, jolt_level);
            return None;
        }
        difference_counts[usize::try_from(difference).unwrap()] += 1;
        jolt_level = lowest_adapter;
    }

    return Some(Box::new(difference_counts));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jolt_differences_small_example() {
        // Setup
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        // Action
        let difference_counts: [u32; 4] = *compute_jolt_differences(&adapters).unwrap();
        // Verification
        assert_eq!(difference_counts[1], 7);
        assert_eq!(difference_counts[2], 0);
        assert_eq!(difference_counts[3], 5);
    }

    #[test]
    fn test_jolt_differences_larger_example() {
        // Setup
        let adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45,
            19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        // Action
        let difference_counts: [u32; 4] = *compute_jolt_differences(&adapters).unwrap();
        // Verification
        assert_eq!(difference_counts[1], 22);
        assert_eq!(difference_counts[2], 0);
        assert_eq!(difference_counts[3], 10);
    }
}