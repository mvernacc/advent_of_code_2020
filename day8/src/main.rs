#[macro_use]
extern crate lazy_static;

use std::{fs, collections::HashSet};
use std::convert::TryFrom;
use regex::Regex;


fn main() {
    let source = fs::read_to_string("./input.txt").unwrap();
    let program = parse_program(&source);
    let (accumulator, term_cond) = run_program(&program);
    assert_eq!(TerminationCondition::RepeatedInstruction, term_cond);
    println!("Immediately before the program would run an instruction a second time, the value in the accumulator is {:}",
        accumulator);
}

enum Operation {
    ACC, // Increases or decreases the accumulator value.
    JMP, // Jumps to a new instruction relative to itself.
    NOP, // No operation.
}

struct Instruction {
    operation: Operation,
    argument: i32,
}

fn parse_program(source: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref RE_SOURCE_LINE: Regex = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();
    }

    let mut program = Vec::<Instruction>::new();

    for line in source.trim().split("\n") {
        let caps = RE_SOURCE_LINE.captures(line).unwrap();
        let argument = caps[2].parse::<i32>().unwrap();
        program.push(
            match &caps[1] {
                "acc" => Instruction { operation: Operation::ACC, argument: argument },
                "jmp" => Instruction { operation: Operation::JMP, argument: argument },
                "nop" => Instruction { operation: Operation::NOP, argument: argument },
                _ => panic!(),
            }
        );
    }

    program
}

#[derive(PartialEq, Debug)]
enum TerminationCondition {
    RepeatedInstruction,
    ReachedEnd,
}

/// Run the program until any instruction is called a second time, or the program terminates by
/// attempting to execute an instruction immediately after the last instruction in the file.
/// Return the value of the accumulator one of the termination conditions happens.
fn run_program(program: &Vec<Instruction>) -> (i32, TerminationCondition) {
    let mut accumulator: i32 = 0;
    let mut location: usize = 0;
    let mut executed_locations = HashSet::<usize>::new();

    loop {
        // Keep track of which instructions we've executed before.
        if executed_locations.contains(&location) {
            return (accumulator, TerminationCondition::RepeatedInstruction);
        }
        else {
            executed_locations.insert(location);
        }
        // Check if we've reached the end of the program.
        if location == program.len() {
            return (accumulator, TerminationCondition::ReachedEnd);
        }

        // Execute this instruction.
        let instruction = &program[location];
        match instruction.operation {
            Operation::ACC => {
                accumulator += instruction.argument;
                location += 1;
            },
            Operation::JMP => location = usize::try_from(
                i32::try_from(location).unwrap() + instruction.argument).unwrap(),
            Operation::NOP => location += 1,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acc_repeat_example () {
        let source = fs::read_to_string("./example_input.txt").unwrap();
        let program = parse_program(&source);
        let (accumulator, term_cond) = run_program(&program);
        assert_eq!(TerminationCondition::RepeatedInstruction, term_cond);
        assert_eq!(5, accumulator);
    }

    #[test]
    fn test_reach_end_example () {
        let source = fs::read_to_string("./example_input.txt").unwrap();
        let mut program = parse_program(&source);
        program[7].operation = Operation::NOP;
        let (accumulator, term_cond) = run_program(&program);
        assert_eq!(TerminationCondition::ReachedEnd, term_cond);
        assert_eq!(8, accumulator);
    }
}
