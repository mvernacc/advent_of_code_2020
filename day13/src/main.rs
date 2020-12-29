use std::fs;

fn main()  {
    let text = fs::read_to_string("./input.txt").unwrap();
    let lines = text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();
    let ready_time = lines[0].parse::<u32>().unwrap();
    let mut buses = Vec::<u32>::new();
    for bus_str in lines[1].split(',') {
        if bus_str == "x" {
            continue;
        } else {
            buses.push(bus_str.parse::<u32>().unwrap());
        }
    }

    // Part 1.
    let (earliest_bus, lowest_wait_time) = find_earliest_bus(ready_time, &buses);
    println!("Part 1: {} * {} = {}",
        earliest_bus, lowest_wait_time, earliest_bus * lowest_wait_time);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_earliest_bus() {
        let buses = vec![7, 13, 59, 31, 19];
        let (earliest_bus, lowest_wait_time) = find_earliest_bus(939, &buses);
        assert_eq!(earliest_bus, 59);
        assert_eq!(lowest_wait_time, 5);
    }
}
