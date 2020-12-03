use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Policy {
    ch: char,
    min: u32,
    max: u32,
}

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    // let lines = contents.trim().split("\n").collect::<Vec<&str>>();

    let re = Regex::new(r"(?m)^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let mut total_count = 0;
    let mut valid_count = 0;
    for cap in re.captures_iter(&text).by_ref() {
        total_count += 1;
    
        // Create a policy and password from the regex matches.
        let policy = Policy {
            ch: cap[3].chars().next().unwrap(),
            min: cap[1].parse::<u32>().unwrap(),
            max: cap[2].parse().unwrap(),
        };
        let password = &cap[4];

        if check_password_policy(&policy, password) {
            valid_count += 1;
        }
    }
    println!("{} / {} passwords are valid.", valid_count, total_count);
}

fn check_password_policy(policy: &Policy, password: &str) -> bool {
    let mut count = 0;
    for ch in password.chars() {
        if ch == policy.ch {
            count += 1;
        }
    }
    policy.min <= count && count <= policy.max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pw_ok() {
        let policy = Policy {
            ch: 'a',
            min: 2,
            max: 4,
        };
        let password = "skdjaaakla";
        let complies_with_policy = check_password_policy(&policy, &password);
        assert!(complies_with_policy);
    }

    #[test]
    fn pw_bad() {
        let policy = Policy {
            ch: 'a',
            min: 2,
            max: 4,
        };
        let password = "skdjkla";
        let complies_with_policy = check_password_policy(&policy, &password);
        assert!(!complies_with_policy);
    }
}