use std::{ fs, collections::HashSet };


fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let active_at_6 = solve_part_1(&text);
    println!("Part 1: {}", active_at_6);
}

fn solve_part_1(text: &str) -> usize {
    // Parse the Conway space from text.
    let mut active_cubes = HashSet::<[i32; 3]>::new();
    for (y, line) in text.trim().split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert([x as i32, y as i32, 0]);
            }
        }
    }

    // Simulate the Conway space for 6 steps.
    for _ in 0..6 {
        active_cubes = *sim_step(&active_cubes);
    }

    active_cubes.len()
}

fn sim_step(active_cubes_old: &HashSet<[i32; 3]>) -> Box<HashSet<[i32; 3]>> {
    let mut active_cubes_new = HashSet::<[i32; 3]>::new();
    let bounds = get_bounds(&active_cubes_old);

    for x in (bounds[0].0 - 1)..(bounds[0].1 + 2) {
        for y in (bounds[1].0 - 1)..(bounds[1].1 + 2) {
            for z in (bounds[2].0 - 1)..(bounds[2].1 + 2) {
                let point = [x, y, z];
                let count = count_active_neighbors(&point, &active_cubes_old);
                if active_cubes_old.contains(&point) && (count == 2 || count == 3) {
                    active_cubes_new.insert(point);
                } else if !active_cubes_old.contains(&point) && (count == 3) {
                    active_cubes_new.insert(point);
                }
            }
        }
    }

    Box::new(active_cubes_new)
}

/// Get the (minimum, maximum) coordinates with active cubes for each dimension of the Conway space.
fn get_bounds(active_cubes: &HashSet<[i32; 3]>) -> [(i32, i32); 3] {
    let mut bounds = [(i32::MAX, i32::MIN); 3];
    let dims: [usize; 3] = [0, 1, 2];
    for &point in active_cubes.iter() {
        for &dim in dims.iter() {
            if point[dim] < bounds[dim].0 { bounds[dim].0 = point[dim]; }
            if point[dim] > bounds[dim].1 { bounds[dim].1 = point[dim]; }
        }
    }
    bounds
}

fn count_active_neighbors(point: &[i32; 3], active_cubes: &HashSet<[i32; 3]>) -> u32 {
    let mut count: u32 = 0; 
    for &dx in &[-1, 0, 1] {
        for &dy in &[-1, 0, 1] {
            for &dz in &[-1, -0, 1] {
                if !(dx == 0 && dy == 0 && dz == 0)
                    && (active_cubes.contains(&[point[0] + dx, point[1] + dy, point[2] + dz]))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let text = ".#.\n..#\n###";
        let active_at_6 = solve_part_1(&text);
        assert_eq!(active_at_6, 112);
    }
}