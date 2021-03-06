use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let valid_passport_count = count_valid_passports(&text);
    println!("{} valid passports.", valid_passport_count);
}

fn count_valid_passports(text: &str) -> u32 {
    let verbose = false;

    let passports = text
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>();

    let mut field_regexes = HashMap::new();
    field_regexes.insert("byr", Regex::new(r"byr:(\d{4})\b").unwrap());
    field_regexes.insert("iyr", Regex::new(r"iyr:(\d{4})\b").unwrap());
    field_regexes.insert("eyr", Regex::new(r"eyr:(\d{4})\b").unwrap());
    field_regexes.insert("hgt", Regex::new(r"hgt:(\d+)(in|cm)\b").unwrap());
    field_regexes.insert("hcl", Regex::new(r"hcl:#[a-f0-9]{6}\b").unwrap());
    field_regexes.insert("ecl", Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap());
    field_regexes.insert("pid", Regex::new(r"pid:\d{9}\b").unwrap());

    let mut valid_passport_count: u32 = 0;
    for passport in passports.iter() {
        if verbose {
            println!("\n{}", passport);
        }

        let mut valid = true;
        for (field, re) in &field_regexes {
            if verbose {
                print!("Checking {}: ", field);
            }
            let cap = match re.captures_iter(passport).next() {
                None => {
                    if verbose {
                        print!("missing, invalid.\n");
                    }
                    valid = false;
                    break;
                },
                Some(cap) => cap,
            };

            if verbose {
                print!("present, ");
            }

            valid = match &field[..] {
                "byr" => {
                    let year = cap[1].parse::<u32>().unwrap();
                    1920 <= year && year <= 2002
                },
                "iyr" => {
                    let year = cap[1].parse::<u32>().unwrap();
                    2010 <= year && year <= 2020
                },
                "eyr" => {
                    let year = cap[1].parse::<u32>().unwrap();
                    2020 <= year && year <= 2030
                },
                "hgt" => {
                    let value = cap[1].parse::<u32>().unwrap();
                    let units = &cap[2];
                    match units {
                        "cm" => 150 <= value && value <= 193,
                        "in" => 59 <= value && value <= 76,
                        _ => false,
                    }
                },
                // Other fields to not need additional validation (beyond matching the regex).
                _ => true,
            };
            if !valid {
                if verbose {
                    print!("invalid.\n");
                }
                break;
            } else if verbose {
                print!("valid.\n");
            }
        }
        if valid {
            if verbose {
                println!("Passport valid.");
            }
            valid_passport_count += 1;
        } else if verbose {
            println!("Passport invalid.")
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

    #[test]
    fn test_example_input_invalid() {
        let text = fs::read_to_string("./example_input_invalid.txt").unwrap();
        let valid_passport_count = count_valid_passports(&text);
        assert_eq!(valid_passport_count, 0);
    }
}