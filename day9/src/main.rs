use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let numbers = text
        .trim()
        .split("\n")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Part 1
    let first_invalid = xmas_find_first_invalid_number(&numbers, 25).unwrap();
    println!("First invalid number: {}", first_invalid);

    // Part 2
    let contig_set = find_contiguous_set_sum(&numbers, first_invalid).unwrap();
    let smallest = contig_set.iter().min().unwrap();
    let largest = contig_set.iter().max().unwrap();
    let weakness = smallest + largest;
    println!("Encryption weakness: {}", weakness);
}

/// Find the first number in a list which is invalid according to the rules of the
/// "XMAS" cypher.
fn xmas_find_first_invalid_number(numbers: &Vec<u64>, preamble_length: usize) -> Option<u64> {
    assert!(numbers.len() > preamble_length);
    for i in preamble_length..numbers.len() {
        if !matches_sum_of_any_pair(&numbers[i-preamble_length..i], numbers[i]) {
            return Some(numbers[i]);
        }
    }
    return None;
}

fn find_contiguous_set_sum(numbers: &[u64], target_value: u64) -> Option<&[u64]> {
    // `running_sums[j]` is the sum of last `j` numbers.
    let mut running_sums = Vec::<u64>::new();
    running_sums.push(numbers[0]);
    if running_sums[0] == target_value {
        return Some(&numbers[0..1]);
    }

    for i in 1..numbers.len() {
        for j in 0..i {
            running_sums[j] += numbers[i];
            running_sums[j] -= numbers[i - j - 1];
            if running_sums[j] == target_value {
                return Some(&numbers[i - j .. i + 1]);
            }
        }
        running_sums.push(running_sums[i - 1] + numbers[0]);
        if running_sums[i] == target_value {
            return Some(&numbers[0 .. i + 1]);
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

    #[test]
    fn test_find_contiguous_set_sum_example() {
        // Setup
        let numbers = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182,
            127, 219, 299, 277, 309, 576];
        // Action
        let contig_set = find_contiguous_set_sum(&numbers, 127).unwrap();
        // Verification
        assert_eq!(contig_set.len(), 4);
        assert_eq!(contig_set[0], 15);
        assert_eq!(contig_set[3], 40);
    }

    #[test]
    fn test_find_contiguous_set_sum_first() {
        // Setup
        let numbers = vec![35, 20, 15];
        // Action
        let contig_set = find_contiguous_set_sum(&numbers, 35).unwrap();
        // Verification
        assert_eq!(contig_set.len(), 1);
        assert_eq!(contig_set[0], 35);
    }
}
