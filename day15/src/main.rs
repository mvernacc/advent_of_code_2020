use std::collections::HashMap;

fn main() {
    let last_number = elf_game(&vec![20, 0, 1, 11, 6, 3], 2020);
    println!("Part 1: {}", last_number);

    let last_number_2 = elf_game(&vec![20, 0, 1, 11, 6, 3], 30000000);
    println!("Part 2: {}", last_number_2);
}


fn elf_game(starting_numbers: &Vec<usize>, end_turn: usize) -> usize {
    // If `turn_last_spoken[i] == k`, then `i` was last spoken on turn `k`.
    // If `turn_last_spoken` does not contain `i`, then `i` has not been spoken before.
    let mut turn_last_spoken = HashMap::<usize, usize>::new();
    let mut turn_number = 1;
    let mut number_spoken_last_turn = 0;
    let mut number_spoken_this_turn = 0;

    for &number in starting_numbers.iter() {
        turn_last_spoken.insert(number, turn_number);
        number_spoken_last_turn = number;
        turn_number += 1;
    }

    while turn_number <= end_turn {
        if !turn_last_spoken.contains_key(&number_spoken_last_turn) {
            number_spoken_this_turn = 0;
        } else {
            number_spoken_this_turn = turn_number - 1 - turn_last_spoken.get(&number_spoken_last_turn).unwrap();
        }
        turn_last_spoken.insert(number_spoken_last_turn, turn_number - 1);
        // println!("Turn {}: {}", turn_number, number_spoken_this_turn);
        turn_number += 1;
        number_spoken_last_turn = number_spoken_this_turn;
    }
    number_spoken_this_turn
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_elf_game_1() {
        let last_number = elf_game(&vec![0, 3, 6], 2020);
        assert_eq!(last_number, 436);
    }
}
