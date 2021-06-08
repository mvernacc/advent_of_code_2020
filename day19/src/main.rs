use std::{collections::HashMap, fs};

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let mut text_parts = text.split("\n\n");
    let rule_strings: Vec<&str> = text_parts.next().unwrap().lines().collect();
    let messages: Vec<&str> = text_parts.next().unwrap().lines().collect();

    let ruleset = parse_rules(rule_strings);
    let rule0 = ruleset.get(&0).unwrap();

    let mut match_count = 0;
    for message in messages {
        if rule0.matches(&ruleset, message) {
            match_count += 1;
        }
    }

    println!("Messages matching rule 0: {}", match_count);
}

struct Rule {
    id: usize,
    subrule_id_lists: Vec<Vec<usize>>,
    literal_match: Option<char>,
}

impl Rule {
    fn matches_start(&self, ruleset: &HashMap<usize, Rule>, s: &str) -> Option<usize> {
        if !self.literal_match.is_none() {
            if s.starts_with(self.literal_match.unwrap()) {
                return Some(1);
            } else {
                return None;
            }
        } else {
            for subrule_id_list in &self.subrule_id_lists {
                let mut position = 0;
                let mut no_match = false;
                for rule_id in subrule_id_list {
                    let rule = ruleset.get(rule_id).unwrap();
                    match rule.matches_start(ruleset, &s[position..]) {
                        None => {
                            no_match = true;
                            break;
                        },
                        Some(match_length) => {
                            position += match_length;
                        },
                    }
                }
                if !no_match {
                    return Some(position);
                }
            }
            return None;
        }
    }

    fn matches(&self, ruleset: &HashMap<usize, Rule>, s: &str) -> bool {
        match self.matches_start(ruleset, s) {
            None => false,
            Some(match_length) => {
                return match_length == s.len();
            }
        }
    }
}

fn parse_rules(rule_strings: Vec<&str>) -> HashMap<usize, Rule> {
    let mut ruleset = HashMap::<usize, Rule>::new();

    for rule_string in rule_strings {
        let substrs = rule_string.split(":").collect::<Vec::<&str>>();
        let rule_id: usize = substrs[0].parse().unwrap();
        let rule_body = substrs[1].trim();
        if rule_body.starts_with('"') {
            ruleset.insert(
                rule_id,
                Rule {
                    id: rule_id,
                    subrule_id_lists: Vec::<Vec::<usize>>::new(),
                    literal_match: Some(rule_body.chars().nth(1).unwrap()),
                }
            );
        } else {
            let mut new_rule = Rule {
                id: rule_id,
                subrule_id_lists: Vec::<Vec::<usize>>::new(),
                literal_match: None,
            };
            let rule_list_strs = rule_body.split("|");
            for rule_list_str in rule_list_strs {
                let subrule_ids: Vec<usize> = rule_list_str
                    .trim()
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                new_rule.subrule_id_lists.push(subrule_ids);
            }
            ruleset.insert(rule_id, new_rule);
        }
    }

    return ruleset;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_first_example() {
        let mut ruleset = HashMap::<usize, Rule>::new();
        ruleset.insert(
            0, Rule {
                id: 0,
                subrule_id_lists: vec![vec![1, 2]],
                literal_match: None,
            }
        );
        ruleset.insert(
            1, Rule {
                id: 1,
                subrule_id_lists: vec![],
                literal_match: Some('a'),
            }
        );
        ruleset.insert(
            2, Rule {
                id: 2,
                subrule_id_lists: vec![vec![1, 3], vec![3, 1]],
                literal_match: None,
            }
        );
        ruleset.insert(
            3, Rule {
                id: 3,
                subrule_id_lists: vec![],
                literal_match: Some('b'),
            }
        );

        assert!(ruleset.get(&1).unwrap().matches(&ruleset, "a"));
        assert!(ruleset.get(&3).unwrap().matches(&ruleset, "b"));
        assert!(ruleset.get(&2).unwrap().matches(&ruleset, "ab"));
        assert!(ruleset.get(&2).unwrap().matches(&ruleset, "ba"));
        assert!(ruleset.get(&0).unwrap().matches(&ruleset, "aab"));
        assert!(ruleset.get(&0).unwrap().matches(&ruleset, "aba"));
    }

    #[test]
    fn test_part_1_parse_rules() {
        let rule_strings = vec![
            "0: 1 2",
            "1: \"a\"",
            "2: 1 3 | 3 1",
            "3: \"b\"",
        ];

        let ruleset = parse_rules(rule_strings);

        assert_eq!(ruleset.len(), 4);
        assert!(ruleset.get(&1).unwrap().matches(&ruleset, "a"));
        assert!(ruleset.get(&3).unwrap().matches(&ruleset, "b"));
        assert!(ruleset.get(&2).unwrap().matches(&ruleset, "ab"));
        assert!(ruleset.get(&2).unwrap().matches(&ruleset, "ba"));
        assert!(ruleset.get(&0).unwrap().matches(&ruleset, "aab"));
        assert!(ruleset.get(&0).unwrap().matches(&ruleset, "aba"));
        
    }
}