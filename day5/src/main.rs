use std::fs;
use intbits::Bits;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let seat_codes = text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();
    
    // Part 1: Find the maximum seat ID of the seat codes in the input list.
    let mut max_seat_id: u32 = 0;
    for seat_code in seat_codes.iter() {
        let seat_id = compute_seat_id(seat_code).unwrap();
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }
    println!("Max seat ID = {}", max_seat_id);

    // Part 2: Find the missing seat ID.
    let mut missing_seat_id: u32 = 0;
    let mut seat_ids = Vec::<u32>::new();
    for seat_code in seat_codes.iter() {
        seat_ids.push(compute_seat_id(seat_code).unwrap());
    }
    seat_ids.sort();
    for i in 0..(seat_ids.len() - 1) {
        if seat_ids[i] + 1 != seat_ids[i + 1] {
            missing_seat_id = seat_ids[i] + 1;
            break;
        }
    }
    println!("Missing seat ID = {}", missing_seat_id);
}

fn compute_seat_id(partition_code: &str) -> Option<u32> {
    match decode_seat(partition_code) {
        None => None,
        Some((row, column)) => Some(row * 8 + column),
    }
}

fn decode_seat(partition_code: &str) -> Option<(u32, u32)> {
    let mut row: u32 = 0;
    let mut column: u32 = 0;

    let row_bits = 7;
    let column_bits = 3;

    if partition_code.len() != row_bits + column_bits {
        return None;
    }

    // Decode the row
    for (i, ch) in partition_code[..row_bits].chars().enumerate() {
        let bit = match ch {
            'F' => false, // Front of the partition
            'B' => true, // Back of the partition
            _ => return None, // `partition_code` is invalid.
        };
        row.set_bit(row_bits - i - 1, bit);
    }

    // Decode the column
    for (i, ch) in partition_code[row_bits..].chars().enumerate() {
        let bit = match ch {
            'L' => false, // Left of the partition
            'R' => true, // Right of the partition
            _ => return None, // `partition_code` is invalid.
        };
        column.set_bit(column_bits - i - 1, bit);
    }
    return Some((row, column));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let (row, column) = decode_seat("FBFBBFFRLR").unwrap();
        assert_eq!(row, 44);
        assert_eq!(column, 5);
    }

    #[test]
    fn test_part1_example2() {
        let (row, column) = decode_seat("BFFFBBFRRR").unwrap();
        assert_eq!(row, 70);
        assert_eq!(column, 7);
    }

    #[test]
    fn test_part1_example3() {
        let (row, column) = decode_seat("FFFBBBFRRR").unwrap();
        assert_eq!(row, 14);
        assert_eq!(column, 7);
    }

    #[test]
    fn test_part1_example4() {
        let (row, column) = decode_seat("BBFFBBFRLL").unwrap();
        assert_eq!(row, 102);
        assert_eq!(column, 4);
    }
}