use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let numbers = text
        .trim()
        .split("\n")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let first_invalid =xmas_find_first_invalid_number(&numbers, 25).unwrap();
    println!("First invalid number: {}", first_invalid);
}


fn xmas_find_first_invalid_number(numbers: &Vec<u64>, preamble_length: usize) -> Option<u64> {
    assert!(numbers.len() > preamble_length);
    for i in preamble_length..numbers.len() {
        if !matches_sum_of_any_pair(&numbers[i-preamble_length..i], numbers[i]) {
            return Some(numbers[i]);
        }
    }
    return None;
}

/// Check if the sum of any pair of numbers in a given list matches a target value.
fn matches_sum_of_any_pair(numbers: &[u64], target_value: u64) -> bool {
    for (i1, x1) in numbers.iter().enumerate() {
        for (_, x2) in numbers[i1..].iter().enumerate() {
            if x1 == x2 { continue; }
            if x1 + x2 == target_value { return true; }
        }
    }
    return false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_to_25_examples() {
        let numbers: Vec<u64> = (1..26).collect();
        assert!(matches_sum_of_any_pair(&numbers, 26));
        assert!(matches_sum_of_any_pair(&numbers, 49));
        assert!(!matches_sum_of_any_pair(&numbers, 100));
        assert!(!matches_sum_of_any_pair(&numbers, 50));
    }

    #[test]
    fn test_find_first_invalid_preable_5_example() {
        let numbers = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182,
            127, 219, 299, 277, 309, 576];
        let result = xmas_find_first_invalid_number(&numbers, 5).unwrap();
        assert_eq!(result, 127);
    }
}
