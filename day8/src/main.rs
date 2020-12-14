#[macro_use]
extern crate lazy_static;

use std::{fs, collections::HashMap};
use std::convert::TryFrom;
use regex::Regex;


fn main() {
    let source = fs::read_to_string("./input.txt").unwrap();
        let program = parse_program(&source);
        let accumulator = run_program_acc_value_on_instruction_repeat(&program);
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

/// Run the program until any instruction is called a second time. Return the value of the accumulator
/// just before this instruction is executed.
fn run_program_acc_value_on_instruction_repeat(program: &Vec<Instruction>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut location: usize = 0;
    let mut exec_count = HashMap::<usize, u32>::new();

    loop {
        // Keep track of which instructions we've executed before.
        if exec_count.contains_key(&location) {
            return accumulator;
        }
        else {
            exec_count.insert(location, 1);
        }

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
        let accumulator = run_program_acc_value_on_instruction_repeat(&program);
        assert_eq!(5, accumulator);
    }
}
