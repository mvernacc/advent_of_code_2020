use std::{ fs, collections::HashMap };
use regex::Regex;
#[macro_use]
extern crate lazy_static;


fn main() {
    let source = fs::read_to_string("./input.txt").unwrap();
    let sum = run_program(&source);
    println!("Part 1: sum of memory values = {}", sum);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_program() {
        let source = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let sum = run_program(&source);
        assert_eq!(sum, 165);
    }

}