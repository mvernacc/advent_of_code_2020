use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn nav_instructions_manhattan_distance(nav_instructions_text: &str) -> PyResult<i32> {
    let nav_instructions = nav_instructions_text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();

    let mut heading: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for instruction in nav_instructions.iter() {
        let action: char = instruction.chars().nth(0).unwrap();
        let arg = instruction[1..].parse::<i32>().unwrap();
        println!("x: {}, y: {}, heading: {}, inst: {}", x, y, heading, instruction);
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
    m.add_function(wrap_pyfunction!(nav_instructions_manhattan_distance, m)?)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_instructions_manhattan_distance_example() {
        let nav_instructions_text = "F10\nN3\nF7\nR90\nF11";
        let result = nav_instructions_manhattan_distance(&nav_instructions_text);
        let dist = result.ok().unwrap();
        assert_eq!(dist, 25);
    }
}
