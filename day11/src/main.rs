use std::{cmp::{min, max}, fs};

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let seating_area = text
        .trim()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    
    // Part 1.
    let count = occupied_seats_steady_state(&seating_area, 1, false);
    println!("Part 1: Number of seats occupied in steady state: {}", count);
    // Part 1.
    let count = occupied_seats_steady_state(&seating_area, 2, false);
    println!("Part 2: Number of seats occupied in steady state: {}", count);
}

fn count_occupied_seats(seating_area: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for row in 0..seating_area.len() {
        for col in 0..seating_area[0].len() {
            if seating_area[row][col] == '#' { count += 1; }
        }
    }
    return count;
}


fn count_occupied_adjacent_seats(seating_area: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut count = 0;
    for i in max(row as i32 - 1, 0) as usize..min(row + 1, seating_area.len() - 1) + 1 {
        for j in max(col as i32 - 1, 0) as usize..min(col + 1, seating_area[0].len() - 1) + 1 {
            if (i == row) && (j == col) { continue; } // Don't count this seat.
            if seating_area[i][j] == '#' { count += 1; }
        }
    }
    return count;
}

fn count_occupied_visible_seats(seating_area: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut count = 0;
    let mut row_cursor: i32;
    let mut col_cursor: i32;
    for direction in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        // Move the cursor in each direction until we find either a seat or the edge of the seating area.
        row_cursor = row as i32;
        col_cursor = col as i32;
        loop {
            // Move the cursor in the search direction.
            row_cursor += direction.0;
            col_cursor += direction.1;
            // Check if cursor has left the seating area bounds.
            if !(row_cursor >= 0 && row_cursor < seating_area.len() as i32
                    && col_cursor >= 0 && col_cursor < seating_area[0].len() as i32) {
                break;
            }
            // Examine the seat under the cursor.
            let cursor_seat = seating_area[row_cursor as usize][col_cursor as usize];
            match cursor_seat {
                'L' => break,
                '#' => { count += 1; break; },
                '.' => continue, // keep looking along this direction.
                _ => panic!() // invalid seating map.
            };
        }
    }
    return count;
}

fn sim_step_1(seating_area_old: &Vec<Vec<char>>, seating_area_new: &mut Vec<Vec<char>>) -> bool {
    let mut changed = false;
    for row in 0..seating_area_old.len() {
        for col in 0..seating_area_old[0].len() {
            let this_seat_old = seating_area_old[row][col];
            let this_seat_new = match this_seat_old {
                '.' => '.',
                'L' => {
                    let count = count_occupied_adjacent_seats(&seating_area_old, row, col);
                    if count == 0 {'#'} else {'L'}
                },
                '#' => {
                    let count = count_occupied_adjacent_seats(&seating_area_old, row, col);
                    if count >= 4 {'L'} else {'#'}
                },
                _ => panic!()
            }; 
            if this_seat_new != this_seat_old { changed = true; }
            seating_area_new[row][col] = this_seat_new;
        }
    }
    return changed
}


fn sim_step_2(seating_area_old: &Vec<Vec<char>>, seating_area_new: &mut Vec<Vec<char>>) -> bool {
    let mut changed = false;
    for row in 0..seating_area_old.len() {
        for col in 0..seating_area_old[0].len() {
            let this_seat_old = seating_area_old[row][col];
            let this_seat_new = match this_seat_old {
                '.' => '.',
                'L' => {
                    let count = count_occupied_visible_seats(&seating_area_old, row, col);
                    if count == 0 {'#'} else {'L'}
                },
                '#' => {
                    let count = count_occupied_visible_seats(&seating_area_old, row, col);
                    if count >= 5 {'L'} else {'#'}
                },
                _ => panic!()
            }; 
            if this_seat_new != this_seat_old { changed = true; }
            seating_area_new[row][col] = this_seat_new;
        }
    }
    return changed
}

/// Solves part 1.
fn occupied_seats_steady_state(seating_area: &Vec<Vec<char>>, part: u32, verbose: bool) -> u32 {
    let mut seating_area_old = seating_area.clone();
    let mut seating_area_new = seating_area.clone();
    let mut changed = true;
    while changed {

        changed = match part {
            1 => sim_step_1(&seating_area_old, &mut seating_area_new),
            2 => sim_step_2(&seating_area_old, &mut seating_area_new),
            _ => panic!()
        };

        if verbose {
            println!{"\n"};
            for line in seating_area_new.iter() {
                println!("{}", line.iter().collect::<String>());
            }
        }
        let temp = seating_area_old;
        seating_area_old = seating_area_new;
        seating_area_new = temp;
    }

    return count_occupied_seats(&seating_area_new);
}


#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[test]
    fn test_sim_example_part_1() {
        // Setup
        let text = fs::read_to_string("./example_input.txt").unwrap();
        let seating_area = text
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();
        
        // Action
        let count = occupied_seats_steady_state(&seating_area, 1, true);

        // Verification
        assert_eq!(count, 37);
    }

    #[test]
    fn test_sim_example_part_2() {
        // Setup
        let text = fs::read_to_string("./example_input.txt").unwrap();
        let seating_area = text
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();
        
        // Action
        let count = occupied_seats_steady_state(&seating_area, 2, true);

        // Verification
        assert_eq!(count, 26);
    }

    #[rstest(filename, row, col, expected_count,
        case("./visible_seats_example_1.txt", 4, 3, 8),
        case("./visible_seats_example_2.txt", 1, 1, 0),
        case("./visible_seats_example_3.txt", 3, 3, 0),
    )]
    fn test_visible_seats_examples(filename: &str, row: usize, col: usize, expected_count: u32) {
        // Setup
        let text = fs::read_to_string(filename).unwrap();
        let seating_area = text
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();
        // Action
        let count = count_occupied_visible_seats(&seating_area, row, col);
        // Verification
        assert_eq!(count, expected_count);
    }
}