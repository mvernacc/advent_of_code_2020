use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Policy1 {
    ch: char,
    min: u32,
    max: u32,
}

struct Policy2 {
    ch: char,
    // Note: these positions index from 0 in the password string.
    pos1: usize,
    pos2: usize,
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    // let lines = contents.trim().split("\n").collect::<Vec<&str>>();

    let re = Regex::new(r"(?m)^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let mut total_count = 0;
    let mut valid_1_count = 0; // Number of valid passwords for part 1.
    let mut valid_2_count = 0; // Number of valid passwords for part 2.
    for cap in re.captures_iter(&text).by_ref() {
        total_count += 1;
    
        // Create a policy and password from the regex matches.
        let policy1 = Policy1 {
            ch: cap[3].chars().next().unwrap(),
            min: cap[1].parse::<u32>().unwrap(),
            max: cap[2].parse::<u32>().unwrap(),
        };
        let policy2 = Policy2 {
            ch: cap[3].chars().next().unwrap(),
            pos1: cap[1].parse::<usize>().unwrap() - 1,
            pos2: cap[2].parse::<usize>().unwrap() - 1,
        };
        let password = &cap[4];

        if check_password_policy_1(&policy1, password) {
            valid_1_count += 1;
        }
        if check_password_policy_2(&policy2, password) {
            valid_2_count += 1;
        }
    }
    println!("Part 1: {} / {} passwords are valid.", valid_1_count, total_count);
    println!("Part 2: {} / {} passwords are valid.", valid_2_count, total_count);
}

/// Check if a password is valid according to the Part 1 policy interpretation.
fn check_password_policy_1(policy: &Policy1, password: &str) -> bool {
    let mut count = 0;
    for ch in password.chars() {
        if ch == policy.ch {
            count += 1;
        }
    }
    policy.min <= count && count <= policy.max
}


/// Check if a password is valid according to the Part 2 policy interpretation.
fn check_password_policy_2(policy: &Policy2, password: &str) -> bool {
    if policy.pos1 >= password.len() || policy.pos1 >= password.len() {
        return false;
    }

    let c1 = password.as_bytes()[policy.pos1] as char;
    let c2 = password.as_bytes()[policy.pos2] as char;
    (c1 == policy.ch) != (c2 == policy.ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pw1_ok() {
        let policy = Policy1 {
            ch: 'a',
            min: 2,
            max: 4,
        };
        let password = "skdjaaakla";
        let complies_with_policy = check_password_policy_1(&policy, &password);
        assert!(complies_with_policy);
    }

    #[test]
    fn pw1_bad() {
        let policy = Policy1 {
            ch: 'a',
            min: 2,
            max: 4,
        };
        let password = "skdjkla";
        let complies_with_policy = check_password_policy_1(&policy, &password);
        assert!(!complies_with_policy);
    }

    #[test]
    fn pw2_example1() {
        let policy = Policy2 {
            ch: 'a',
            pos1: 1 - 1,
            pos2: 3 - 1,
        };
        let password = "abcde";
        let complies_with_policy = check_password_policy_2(&policy, &password);
        assert!(complies_with_policy);
    }

    #[test]
    fn pw2_example2() {
        let policy = Policy2 {
            ch: 'b',
            pos1: 1 - 1,
            pos2: 3 - 1,
        };
        let password = "cdefg";
        let complies_with_policy = check_password_policy_2(&policy, &password);
        assert!(!complies_with_policy);
    }

    #[test]
    fn pw2_example3() {
        let policy = Policy2 {
            ch: 'c',
            pos1: 2 - 1,
            pos2: 9 - 1,
        };
        let password = "ccccccccc";
        let complies_with_policy = check_password_policy_2(&policy, &password);
        assert!(!complies_with_policy);
    }
}