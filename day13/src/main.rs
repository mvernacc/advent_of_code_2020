use std::fs;
use num_integer::{Integer, ExtendedGcd};

fn main()  {
    let text = fs::read_to_string("./input.txt").unwrap();
    let lines = text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();
    let ready_time = lines[0].parse::<u32>().unwrap();
    let mut buses = Vec::<u32>::new();
    let mut offsets = Vec::<u32>::new();
    for (i, bus_str) in lines[1].split(',').enumerate() {
        if bus_str == "x" {
            continue;
        } else {
            buses.push(bus_str.parse::<u32>().unwrap());
            offsets.push(i as u32);
        }
    }

    // Part 1.
    let (earliest_bus, lowest_wait_time) = find_earliest_bus(ready_time, &buses);
    println!("Part 1: {} * {} = {}",
        earliest_bus, lowest_wait_time, earliest_bus * lowest_wait_time);
    // Part 2.
    let earliest = find_earliest_part_2(&buses, &offsets);
    println!("Part 2: {}", earliest);

}


/// Find the (bus ID, wait time) of the earliest bus after `ready_time`.
fn find_earliest_bus(ready_time: u32, buses: &Vec<u32>) -> (u32, u32) {
    let mut lowest_wait_time = u32::max_value();
    let mut earliest_bus: u32 = 0;

    for bus in buses.iter() {
        let wait_time_this_bus = bus - (ready_time % bus);
        if wait_time_this_bus < lowest_wait_time {
            lowest_wait_time = wait_time_this_bus;
            earliest_bus = *bus;
        }
    }
    assert_ne!(earliest_bus, 0);

    (earliest_bus, lowest_wait_time)
}

/// period_c, phase_c, x
fn periodic_combination(period_a: i128, period_b: i128,
        phase_a: i128, phase_b: i128) -> Option<(i128, i128)> {
    // Following https://math.stackexchange.com/a/38128593
    let ExtendedGcd { gcd, x, .. } = period_a.extended_gcd(&period_b);
    let s = x;

    let phase_difference = phase_a - phase_b;
    let (pd_mult, pd_remainder) = phase_difference.div_mod_floor(&gcd);
    if pd_remainder != 0 {
        // "A and B will never land on their reference points at the same step"
        return None;
    }
    let period_c = period_a / gcd * period_b;
    let phase_c = (phase_a - s * pd_mult * period_a).mod_floor(&period_c);

    return Some((period_c, phase_c))
}

fn find_earliest_part_2(buses: &Vec<u32>, offsets: &Vec<u32>) -> u128 {
    assert_eq!(buses.len(), offsets.len());

    let mut period = buses[0] as i128;
    let mut phase = offsets[0] as i128;

    for (bus, offset) in buses[1..].iter().zip(offsets[1..].iter()) {
        let result = periodic_combination(
            period, *bus as i128, phase, *offset as i128).unwrap();
        period = result.0; phase = result.1;
    }
    return (period - phase) as u128;
}


#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[test]
    fn test_mod() {
        let x = 5;
        assert_eq!(-1, x.mod_floor(&-3));
    }

    #[test]
    fn test_find_earliest_bus() {
        let buses = vec![7, 13, 59, 31, 19];
        let (earliest_bus, lowest_wait_time) = find_earliest_bus(939, &buses);
        assert_eq!(earliest_bus, 59);
        assert_eq!(lowest_wait_time, 5);
    }

    #[rstest(period_a, period_b, phase_a, phase_b, correct_period_c, correct_phase_c,
        case(9, 15, 0, 3, 45, 18),
        case(2, 3, -1, -2, 6, 1),
        case(2, 3, 0, 1, 6, 6-2),
        // case(3, 2, -2, -1, 6, 5),
        case(17, 13, 0, 2, 221, 221-102),
    )]
    fn test_periodic_combination(period_a: i128, period_b: i128, phase_a: i128, phase_b: i128,
            correct_period_c: i128, correct_phase_c: i128) {
        let (period_c, phase_c) = periodic_combination(period_a, period_b, phase_a, phase_b).unwrap();
        assert_eq!(period_c, correct_period_c);
        assert_eq!(phase_c, correct_phase_c);
    }

    #[test]
    fn test_part_2() {
        let buses = vec![7, 13, 59, 31, 19];
        let offsets = vec![0, 1, 4, 6, 7];
        let earliest = find_earliest_part_2(&buses, &offsets);
        assert_eq!(earliest, 1068781);
    }

    #[test]
    fn test_part_2_ex1() {
        let buses = vec![17, 13, 19];
        let offsets = vec![0, 2, 3];
        let earliest = find_earliest_part_2(&buses, &offsets);
        assert_eq!(earliest, 3417);
    }

}
