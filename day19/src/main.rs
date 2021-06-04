fn main() {
    println!("Hello, world!");
}

struct Rule<'a> {
    subrule_lists: Vec<Vec<&'a Rule<'a>>>,
    literal_match: Option<char>,
}

impl Rule<'_> {
    fn matches_start(&self, s: &str) -> Option<usize> {
        if !self.literal_match.is_none() {
            if s.starts_with(self.literal_match.unwrap()) {
                return Some(1);
            } else {
                return None;
            }
        } else {
            for subrule_list in &self.subrule_lists {
                let mut position = 0;
                let mut no_match = false;
                for rule in subrule_list {
                    match rule.matches_start(&s[position..]) {
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

    fn matches(&self, s: &str) -> bool {
        match self.matches_start(s) {
            None => false,
            Some(match_length) => {
                return match_length == s.len();
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_first_example() {
        let rule1 = Rule {
            subrule_lists: Vec::<Vec::<&Rule>>::new(),
            literal_match: Some('a'),
        };
        let rule3 = Rule {
            subrule_lists: Vec::<Vec::<&Rule>>::new(),
            literal_match: Some('b'),
        };
        let rule2 = Rule {
            subrule_lists: vec![vec![&rule1, &rule3], vec![&rule3, &rule1]],
            literal_match: None,
        };
        let rule0 = Rule {
            subrule_lists: vec![vec![&rule1, &rule2]],
            literal_match: None,
        };

        assert!(rule1.matches("a"));
        assert!(rule3.matches("b"));
        assert!(rule2.matches("ab"));
        assert!(rule2.matches("ba"));
        assert!(rule0.matches("aab"));
        assert!(rule0.matches("aba"));
    }
}