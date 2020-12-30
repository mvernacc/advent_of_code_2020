use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let error_rate = solve_part_1(&text);
    println!("Part 1: {}", error_rate);
}


fn is_in_any_range(value: u32, ranges: &Vec<(u32, u32)>) -> bool {
    for &range in ranges.iter() {
        if range.0 <= value && value <= range.1 { return true; }
    }
    return false;
}

fn solve_part_1(text: &str) -> u32 {
    let paragraphs = text.split("\n\n").collect::<Vec<&str>>();
    let fields_text = paragraphs[0];

    let mut ranges = Vec::<(u32, u32)>::new();
    for line in fields_text.split("\n") {
        let name_and_ranges = line.split(": ").collect::<Vec<&str>>();
        for range_str in name_and_ranges[1].split(" or ") {
            let numbers = range_str.split("-").collect::<Vec<&str>>();
            ranges.push((
                numbers[0].parse::<u32>().unwrap(),
                numbers[1].parse::<u32>().unwrap()
            ));
        }
    }

    let nearby_tickets_lines = paragraphs[2].trim().split("\n").collect::<Vec<&str>>();
    let mut nearby_tickets_values = Vec::<Vec<u32>>::new();
    for line in nearby_tickets_lines[1..].iter() {
        nearby_tickets_values.push(
            line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }

    let mut error_rate = 0;
    for ticket in nearby_tickets_values.iter() {
        for &value in ticket {
            if !is_in_any_range(value, &ranges) {
                error_rate += value;
            }
        }
    }
    error_rate
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let text = fs::read_to_string("example_input.txt").unwrap();
        let error_rate = solve_part_1(&text);
        assert_eq!(error_rate, 71);
    }
}