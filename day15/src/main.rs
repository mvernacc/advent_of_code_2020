fn main() {
    println!("Hello, world!");
    let last_number = elf_game(&vec![20, 0, 1, 11, 6, 3]);
    println!("Part 1: {}", last_number);
}

const END_TURN: usize = 2020;

fn elf_game(starting_numbers: &Vec<usize>) -> usize {
    // If `turn_last_spoken[i] == k != 0`, then `i` was last spoken on turn `k`.
    // If `turn_last_spoken[i]` then `i` has not been spoken before.
    let mut turn_last_spoken = [0 as usize; END_TURN as usize];
    let mut turn_number = 1;
    let mut number_spoken_last_turn = 0;
    let mut number_spoken_this_turn = 0;

    for &number in starting_numbers.iter() {
        turn_last_spoken[number] = turn_number;
        number_spoken_last_turn = number;
        turn_number += 1;
    }

    while turn_number <= END_TURN {
        if turn_last_spoken[number_spoken_last_turn] == 0 {
            number_spoken_this_turn = 0;
        } else {
            number_spoken_this_turn = turn_number - 1 - turn_last_spoken[number_spoken_last_turn];
        }
        turn_last_spoken[number_spoken_last_turn] = turn_number - 1;
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
        let last_number = elf_game(&vec![0, 3, 6]);
        assert_eq!(last_number, 436);
    }
}
