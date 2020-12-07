use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let valid_passport_count = count_valid_passports(&text);
    println!("{} valid passports.", valid_passport_count);
}

fn count_valid_passports(text: &str) -> u32 {
    let passports = text
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>();
    
    let required_fields = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid"
    ];
    let mut field_regexes = HashMap::new();
    for field in required_fields.iter() {
        field_regexes.insert(
            field,
            Regex::new(&format!("{}:\\S+", field)).unwrap()
        );
    }

    let mut valid_passport_count: u32 = 0;
    for passport in passports {
        let mut valid = true;
        for field in required_fields.iter() {
            if !field_regexes[field].is_match(passport) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_passport_count += 1;
        }
    }
    valid_passport_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let text = fs::read_to_string("./example_input.txt").unwrap();
        let valid_passport_count = count_valid_passports(&text);
        assert_eq!(valid_passport_count, 2);
    }
}