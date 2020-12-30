use std::{ fs, ops, collections::HashMap };
use regex::Regex;
#[macro_use]
extern crate lazy_static;


fn main() {
    let source = fs::read_to_string("./input.txt").unwrap();
    let sum = run_program(&source);
    println!("Part 1: sum of memory values = {}", sum);
}

/// Mask a value according to the part 1 rules.
/// `zeros_mask` has a 0 at every position to be overwritten with 0.
/// `ones_mask` has a 1 at every position to be overwritten with 1.
/// Using generic types is not necessary to solve the problem, but I wanted to practice using them.
fn mask_value<T: ops::BitAndAssign + ops::BitOr<Output = T>>(value: T,  zeros_mask: T, ones_mask: T) -> T {
    let mut masked_value: T = value | ones_mask;
    masked_value &= zeros_mask;
    masked_value
}

fn str_to_bitmasks(s: &str) -> (u64, u64) {
    assert!(s.len() <= 64);
    let mut ones_mask: u64 = 0; 
    let mut zeros_mask: u64 = !0;

    for (i, c) in s.chars().rev().enumerate() {
        match c {
            '0' => zeros_mask = !(zeros_mask & (1 << i)),
            '1' => ones_mask |= 1 << i,
            'X' => {},
            _ => unreachable!(),
        }
    } 
    
    (zeros_mask, ones_mask)
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
    let mut zeros_mask: u64 = !0;
    let mut ones_mask: u64 = 0;
    let mask36: u64 = !0 >> 64 - 36;
    // println!("0b{:064b}", mask36);

    for inst in instructions.iter() {
        if inst.starts_with("mask") {
            let mask_str = RE_MASK.captures(inst).unwrap()
                                  .get(1).unwrap().as_str();
            let new_masks = str_to_bitmasks(&mask_str);
            zeros_mask = new_masks.0; ones_mask = new_masks.1;

        } else if inst.starts_with("mem") {
            let caps = RE_MEM.captures(inst).unwrap();
            let address = caps.get(1).unwrap().as_str()
                              .parse::<u64>().unwrap();
            let value = caps.get(2).unwrap().as_str()
                              .parse::<u64>().unwrap() & mask36;
            if address > mask36 { panic!(); }
            let masked_value = mask_value(value, zeros_mask, ones_mask);
            memory.insert(address, masked_value);
        } else {
            unreachable!();
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod test {
    use rstest::*;
    use super::*;

    #[test]
    fn test_mask_value() {
        let value: u64 = 11;
        let zeros_mask: u64 = !(2);
        let ones_mask: u64 = 64;
        let masked_value = mask_value(value, zeros_mask, ones_mask);
        assert_eq!(masked_value, 73);
    }

    #[test]
    fn test_str_to_bitmasks() {
        let s = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let (zeros_mask, ones_mask) = str_to_bitmasks(&s);
        assert_eq!(zeros_mask, !2);
        assert_eq!(ones_mask, 64);
    }

    #[rstest(value, mask_str, correct_masked_value,
        case(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 73),
        case(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101),
        case(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 64),
    )]
    fn test_mask_from_string(value: u64, mask_str: &str, correct_masked_value: u64) {
        let (zeros_mask, ones_mask) = str_to_bitmasks(&mask_str);
        let masked_value = mask_value(value, zeros_mask, ones_mask);
        assert_eq!(masked_value, correct_masked_value);
    }

    #[test]
    fn test_run_program() {
        let source = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let sum = run_program(&source);
        assert_eq!(sum, 165);
    }

}