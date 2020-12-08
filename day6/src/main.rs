use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let group_strings = text
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut groups_answers = Vec::<Vec::<&str>>::new();
    for group_string in group_strings {
        groups_answers.push(
            group_string.split("\n").collect::<Vec<&str>>());
    }

    let mut group_count_sum_1 = 0; // answer for Part 1
    let mut group_count_sum_2 = 0; // answer for Part 2
    for group_answers in groups_answers {
        group_count_sum_1 += count_group_questions_with_a_yes(&group_answers);
        group_count_sum_2 += count_group_questions_all_yes(&group_answers);
    }
    println!("Part 1 group count sum: {}", group_count_sum_1);
    println!("Part 2 group count sum: {}", group_count_sum_2);
}

fn letter_to_index(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

/// Count the number of questions for which at least one person in the group answered yes.
fn count_group_questions_with_a_yes(group_answers: &Vec<&str>) -> u32 {
    // For example, `question_has_a_yes[letter_to_index('b')]` will be set to true
    // IFF at least one personin the group answered yes to question 'b', i.e. if
    // `'b'` appears in the list of strings.
    let mut question_has_a_yes: [bool; 26] = [false; 26];
    for person_answers in group_answers {
        for c in person_answers.chars() {
            question_has_a_yes[letter_to_index(c)] = true;
        }
    }
    let mut yes_count = 0;
    for b in &question_has_a_yes {
        if *b {
            yes_count += 1;
        }
    }
    yes_count
}

/// Count the number of questions for which every person in the group answered yes.
fn count_group_questions_all_yes(group_answers: &Vec<&str>) -> u32 {
    let number_of_people = group_answers.len() as u32;
    // For example, `question_yes_counts[letter_to_index('b')]` will be number of
    // people in the group who answered yes to question 'b'.
    let mut question_yes_counts: [u32; 26] = [0; 26];
    for person_answers in group_answers {
        for c in person_answers.chars() {
            question_yes_counts[letter_to_index(c)] += 1;
        }
    }

    let mut all_yes_count = 0;  // number of questions for which everyone answered yes.
    for count in &question_yes_counts {
        if *count == number_of_people {
            all_yes_count += 1;
        }
    }
    all_yes_count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for `count_group_questions_with_a_yes`
    #[test]
    fn test_count_group_with_yes_example_1() {
        let group_answers: Vec<&str> = vec!["abc"];
        let yes_count = count_group_questions_with_a_yes(&group_answers);
        assert_eq!(yes_count, 3);
    }

    #[test]
    fn test_count_group_with_yes_example_2() {
        let group_answers: Vec<&str> = vec!["a", "b", "c"];
        let yes_count = count_group_questions_with_a_yes(&group_answers);
        assert_eq!(yes_count, 3);
    }

    #[test]
    fn test_count_group_with_yes_example_3() {
        let group_answers: Vec<&str> = vec!["ab" , "ac"];
        let yes_count = count_group_questions_with_a_yes(&group_answers);
        assert_eq!(yes_count, 3);
    }

    #[test]
    fn test_count_group_with_yes_example_4() {
        let group_answers: Vec<&str> = vec!["a", "a", "a", "a"];
        let yes_count = count_group_questions_with_a_yes(&group_answers);
        assert_eq!(yes_count, 1);
    }

    // Tests for `count_group_questions_all_yes`
    #[test]
    fn test_count_group_all_yes_example_1() {
        let group_answers: Vec<&str> = vec!["abc"];
        let yes_count = count_group_questions_all_yes(&group_answers);
        assert_eq!(yes_count, 3);
    }

    #[test]
    fn test_count_group_all_yes_example_2() {
        let group_answers: Vec<&str> = vec!["a", "b", "c"];
        let yes_count = count_group_questions_all_yes(&group_answers);
        assert_eq!(yes_count, 0);
    }

    #[test]
    fn test_count_group_all_yes_example_3() {
        let group_answers: Vec<&str> = vec!["ab" , "ac"];
        let yes_count = count_group_questions_all_yes(&group_answers);
        assert_eq!(yes_count, 1);
    }

    #[test]
    fn test_count_group_all_yes_example_4() {
        let group_answers: Vec<&str> = vec!["a", "a", "a", "a"];
        let yes_count = count_group_questions_all_yes(&group_answers);
        assert_eq!(yes_count, 1);
    }
}