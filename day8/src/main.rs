#[macro_use]
extern crate lazy_static;

use std::{fs, collections::HashSet};
use std::convert::TryFrom;
use regex::Regex;


fn main() {
    let source = fs::read_to_string("./input.txt").unwrap();
    let program = parse_program(&source);

    // Part 1.
    let (accumulator, term_cond) = run_program(&program);
    assert_eq!(TerminationCondition::RepeatedInstruction, term_cond);
    println!("Immediately before the program would run an instruction a second time, the value in the accumulator is {:}",
        accumulator);

    // Part 2.
    let (accumulator, i_corrupt) = fix_corrupt_instruction(&program);
    println!("If instuction {:} is fixed, the program terminates with accumulator value {:}.",
        i_corrupt, accumulator);
}

#[derive(Clone, PartialEq, Debug)]
enum Operation {
    ACC, // Increases or decreases the accumulator value.
    JMP, // Jumps to a new instruction relative to itself.
    NOP, // No operation.
}

#[derive(Clone)]
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

/// Solve Part 2.
/// Returns (accumulator value at termination for the fixed program, index of corrupt instruction). 
/// Computational effort: This should run on O(n^2) time, where n is the number of instructions in the program.
///     - Need to check O(n) JMP/NOP substitutions.
///     - Checking each substitution requires iterating through O(n) instructions.
fn fix_corrupt_instruction(original_program: &Vec<Instruction>) -> (i32, usize) {
    for i in 0..original_program.len() {
        if original_program[i].operation == Operation::ACC {
            continue;
        }

        // Change instruction `i` from JMP -> NOP or visa versa.
        let mut new_program = original_program.clone();
        if new_program[i].operation == Operation::NOP {
            new_program[i].operation = Operation::JMP;
        } else {
            new_program[i].operation = Operation::NOP;
        }

        // See if the modified program reaches its end.
        let (accumulator, term_cond) = run_program(&new_program);
        if term_cond == TerminationCondition::ReachedEnd {
            println!("End reached by changing instruction {:} to {:?}",
                i, new_program[i].operation);
            return (accumulator, i);
        }
    }
    println!("Could not find a NOP->JMP or JMP->NOP change which makes the program reach its end.");
    panic!();
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

    #[test]
    fn test_fix_example () {
        let source = fs::read_to_string("./example_input.txt").unwrap();
        let program = parse_program(&source);
        let (accumulator, i_corrupt) = fix_corrupt_instruction(&program);
        assert_eq!(7, i_corrupt);
        assert_eq!(8, accumulator);
    }
}
