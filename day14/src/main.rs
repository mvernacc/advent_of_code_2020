use std::{ fs, collections::HashMap };
use regex::Regex;
#[macro_use]
extern crate lazy_static;


fn main() {
    let source = fs::read_to_string("./input.txt").unwrap();
    let sum = run_program(&source);
    println!("Part 1: sum of memory values = {}", sum);
    let sum_2 = run_program_2(&source);
    println!("Part 2: sum of memory values = {}", sum_2);
}

/// Runs the part 1 program and returns the sum of all values in memory.
fn run_program(source: &str) -> u64 {
    lazy_static! {
        static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
        static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let instructions = source
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut memory = HashMap::<u64, u64>::new();
    let mut mask: u64 = !0;
    let mut imprint: u64 = 0;

    for inst in instructions.iter() {
        if inst.starts_with("mask") {
            let mask_str = RE_MASK.captures(inst).unwrap()
                                  .get(1).unwrap().as_str();
            mask = !0;
            imprint = 0;
            for (index, element) in mask_str.chars().rev().enumerate() {
                match element {
                    'X' => {}
                    '0' => {
                        mask &= !((1 as u64) << index);
                    }
                    '1' => {
                        mask &= !((1 as u64) << index);
                        imprint |= (1 as u64) << index;
                    }
                    _ => unreachable!(),
                }
            }
        } else if inst.starts_with("mem") {
            let caps = RE_MEM.captures(inst).unwrap();
            let address = caps.get(1).unwrap().as_str()
                              .parse::<u64>().unwrap();
            let value = caps.get(2).unwrap().as_str()
                              .parse::<u64>().unwrap();
            memory.insert(address, (value & mask) | imprint);
        } else {
            unreachable!();
        }
    }

    memory.values().sum()
}


/// Runs the part 2 program and returns the sum of all values in memory.
fn run_program_2(source: &str) -> u64 {
    lazy_static! {
        static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
        static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let instructions = source
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut memory = HashMap::<u64, u64>::new();
    let mut imprint: u64 = 0;
    let mut not_floating_mask: u64 = !0; // 0 at each floating bit.
    let mut floating_bit_positions: Vec<u64> = Vec::new();

    for inst in instructions.iter() {
        if inst.starts_with("mask") {
            let mask_str = RE_MASK.captures(inst).unwrap()
                                  .get(1).unwrap().as_str();
            imprint = 0;
            not_floating_mask = !0;
            floating_bit_positions = Vec::new();
            for (index, element) in mask_str.chars().rev().enumerate() {
                match element {
                    'X' => {
                        floating_bit_positions.push(index as u64);
                        not_floating_mask &= !(1 << index);
                    }
                    '0' => {}
                    '1' => {
                        imprint |= (1 as u64) << index;
                    }
                    _ => unreachable!(),
                }
            }
        } else if inst.starts_with("mem") {
            let caps = RE_MEM.captures(inst).unwrap();
            let raw_address = caps.get(1).unwrap().as_str()
                              .parse::<u64>().unwrap();
            let value = caps.get(2).unwrap().as_str()
                              .parse::<u64>().unwrap();
    
            // From the `raw_address`, set all floating bits to 0 and all one bits to 1.
            let base_address = (raw_address & not_floating_mask) | imprint;

            for options in 0..(1 << floating_bit_positions.len()) {
                let mut floating_imprint: u64 = 0;
                for (index, position) in floating_bit_positions.iter().enumerate() {
                    if options & (1 << index) > 0 {
                        floating_imprint |= 1 << position;
                    }
                }
                memory.insert(base_address | floating_imprint, value);
            }
        } else {
            unreachable!();
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() {
        let source = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let sum = run_program(&source);
        assert_eq!(sum, 165);
    }

    #[test]
    fn test_run_program_2() {
        let source = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let sum = run_program_2(&source);
        assert_eq!(sum, 208);
    }

}