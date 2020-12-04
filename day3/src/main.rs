use std::fs;
use std::convert::TryFrom;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let terrain = load_terrain(&contents);

    // Part 1.
    let tree_count_31 = count_trees(0, 3, 1, &terrain);
    println!("Part 1: trees on path = {}", tree_count_31);

    // Part 2.
    let tree_count_11 = count_trees(0, 1, 1, &terrain);
    let tree_count_51 = count_trees(0, 5, 1, &terrain);
    let tree_count_71 = count_trees(0, 7, 1, &terrain);
    let tree_count_12 = count_trees(0, 1, 2, &terrain);
    let product = tree_count_11 * tree_count_31 * tree_count_51 * tree_count_71 * tree_count_12;
    println!("Part 2: product of the tree counts = {}", product);
}

/// Parse the content of the input file.
fn load_terrain(text: &str) -> Vec<Vec<char>> {
    text
        .trim()
        .split("\n")
        .map( |x| x.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

/// Wrap the column position `x`.
/// This always returns a value in [0, width).
fn wrap_column(x: i32, width: usize) -> usize {
    let xmod = x % i32::try_from(width).unwrap();
    if xmod < 0 {
        return usize::try_from(i32::try_from(width).unwrap() + xmod).unwrap();
    }
    usize::try_from(xmod).unwrap()
}

/// Count the number of trees encountered on the toboggan's path through the terrain.
/// `starting_col` is the column of the first row which the toboggan starts at.
/// `right_step` is how many spaces the toboggan moves right on each time-step.
///     Negative values make the toboggan move left.
/// `down_step` is how many spaces the toboggan moves down on each time-step.
fn count_trees(
        starting_col: i32, right_step: i32, down_step: usize,
        terrain: &Vec<Vec<char>>) -> u32 {
    let width = terrain[0].len();  // the width of the terrain pattern.
    let mut tree_count = 0;
    // The position of the toboggan in the terrain pattern.
    let mut col: usize = wrap_column(starting_col, width);
    let mut row: usize = 0;
    while row < terrain.len() {
        if terrain[row][col] == '#' {
            tree_count += 1;
        }
        row += down_step;
        col = wrap_column(i32::try_from(col).unwrap() + right_step, width);
    }
    tree_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let contents = fs::read_to_string("./test_input.txt").unwrap();
        let terrain = load_terrain(&contents);
        let tree_count = count_trees(0, 3, 1, &terrain);
        assert_eq!(tree_count, 7);
    }
    #[test]
    fn test_example_input_11() {
        let contents = fs::read_to_string("./test_input.txt").unwrap();
        let terrain = load_terrain(&contents);
        let tree_count = count_trees(0, 1, 1, &terrain);
        assert_eq!(tree_count, 2);
    }

    #[test]
    fn test_example_input_51() {
        let contents = fs::read_to_string("./test_input.txt").unwrap();
        let terrain = load_terrain(&contents);
        let tree_count = count_trees(0, 5, 1, &terrain);
        assert_eq!(tree_count, 3);
    }

    #[test]
    fn test_example_input_71() {
        let contents = fs::read_to_string("./test_input.txt").unwrap();
        let terrain = load_terrain(&contents);
        let tree_count = count_trees(0, 7, 1, &terrain);
        assert_eq!(tree_count, 4);
    }

    #[test]
    fn test_example_input_12() {
        let contents = fs::read_to_string("./test_input.txt").unwrap();
        let terrain = load_terrain(&contents);
        let tree_count = count_trees(0, 1, 2, &terrain);
        assert_eq!(tree_count, 2);
    }
}