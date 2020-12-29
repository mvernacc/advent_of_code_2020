use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Solves according to rules for part 1.
#[pyfunction(verbose=false)]
#[text_signature = "(nav_instructions_text, verbose, /)"]
fn nav_instructions_manhattan_distance_1(nav_instructions_text: &str, verbose: bool) -> PyResult<i32> {
    let nav_instructions = nav_instructions_text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut heading: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for instruction in nav_instructions.iter() {
        if verbose {
            println!("x: {}, y: {}, heading: {}, inst: {}", x, y, heading, instruction);
        }

        let action: char = instruction.chars().nth(0).unwrap();
        let arg = instruction[1..].parse::<i32>().unwrap();
        match action {
            'N' => y += arg,
            'S' => y -= arg,
            'E' => x += arg,
            'W' => x -= arg,
            'L' => {
                heading += arg;
                heading = wrap_heading(heading);
            },
            'R' => {
                heading -= arg;
                heading = wrap_heading(heading);
            }
            'F' => {
                x += arg * int_cosine(heading);
                y += arg * int_sine(heading);
            }
            _ => panic!()
        }
    }

    Ok(x.abs() + y.abs())
}

/// Solves according to rules for part 2.
#[pyfunction(verbose=false)]
#[text_signature = "(nav_instructions_text, verbose, /)"]
fn nav_instructions_manhattan_distance_2(nav_instructions_text: &str, verbose: bool) -> PyResult<i32> {
    let nav_instructions = nav_instructions_text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut ship: (i32, i32) = (0, 0);
    // Waypoint (x, y) position is relative to the ship.
    let mut waypoint: (i32, i32) = (10, 1);
    for instruction in nav_instructions.iter() {
        if verbose {
            println!("ship: {}, {};    waypoint {}, {};    inst: {}",
            ship.0, ship.1, waypoint.0, waypoint.1, instruction);
        }

        let action: char = instruction.chars().nth(0).unwrap();
        let arg = instruction[1..].parse::<i32>().unwrap();
        match action {
            'N' => waypoint.1 += arg,
            'S' => waypoint.1 -= arg,
            'E' => waypoint.0 += arg,
            'W' => waypoint.0 -= arg,
            'L' => {
                waypoint = int_rotate_point(waypoint, arg);
            },
            'R' => {
                waypoint = int_rotate_point(waypoint, -1 * arg);
            }
            'F' => {
                ship.0 += arg * waypoint.0;
                ship.1 += arg * waypoint.1;
            }
            _ => panic!()
        }
    }

    Ok(ship.0.abs() + ship.1.abs())
}

/// Rotate a point (x, y) about (0, 0) by `angle` degrees counterclockwise.
fn int_rotate_point(point: (i32, i32), angle: i32) -> (i32, i32) {
    let wrapped_angle = wrap_heading(angle);
    let cos = int_cosine(wrapped_angle);
    let sin = int_sine(wrapped_angle);
    (
        point.0 * cos - point.1 * sin,
        point.0 * sin + point.1 * cos
    )
}

fn wrap_heading(heading: i32) -> i32 {
     let mut new_heading = heading % 360;
    if new_heading < 0 { new_heading += 360; }
    new_heading
}

fn int_cosine(angle: i32) -> i32 {
    match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!()

    }
}

fn int_sine(angle: i32) -> i32 {
    match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!()

    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn day12(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(nav_instructions_manhattan_distance_1, m)?)?;
    m.add_function(wrap_pyfunction!(nav_instructions_manhattan_distance_2, m)?)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_instructions_manhattan_distance_1_example() {
        let nav_instructions_text = "F10\nN3\nF7\nR90\nF11";
        let result = nav_instructions_manhattan_distance_1(&nav_instructions_text, true);
        let dist = result.ok().unwrap();
        assert_eq!(dist, 25);
    }

    #[test]
    fn nav_instructions_manhattan_distance_2_example() {
        let nav_instructions_text = "F10\nN3\nF7\nR90\nF11";
        let result = nav_instructions_manhattan_distance_2(&nav_instructions_text, true);
        let dist = result.ok().unwrap();
        assert_eq!(dist, 286);
    }
}
