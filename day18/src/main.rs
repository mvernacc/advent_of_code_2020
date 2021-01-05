use std::fs;

fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();

    // Part 1.
    let sum: u64 = text
        .trim()
        .split("\n")
        .map(|expr| evaluate_expression(&expr))
        .sum();
    println!("Part 1: {}", sum);

    // Part 2.
    let operator_precedence = vec!['+', '*']; // Part 2's operator precedence.
    let sum: i64 = text
        .trim()
        .split("\n")
        .map(|expr| evaluate_infix_expression(&expr, &operator_precedence))
        .sum();
    println!("Part 2: {}", sum);
}


/// Returns (expression before parens, expression in parens)
fn get_subexpressions_before_and_in_parens(expression: &str) -> (&str, &str) {
    assert_eq!(expression.chars().last().unwrap(), ')');

    let mut matching_paren_position = 0;
    let mut paren_count = 0;
    for (i, c) in expression.char_indices().rev() {
        match c {
            ')' => paren_count += 1,
            '(' => {
                paren_count -= 1;
                if paren_count == 0 {
                    matching_paren_position = i;
                    break
                }
            }
            _ => {}
        };
    }

    (expression.get(..matching_paren_position).unwrap(),
     expression.get(matching_paren_position + 1..expression.len() - 1).unwrap())
}


fn evaluate_expression(expression: &str) -> u64 {
    let last_char = expression.chars().last().unwrap();
    if last_char == ')' {
        let (before_expression, parens_expression) = get_subexpressions_before_and_in_parens(&expression);
        let parens_value = evaluate_expression(parens_expression);
        if before_expression.len() == 0 {
            return parens_value;
        }

        let operator = before_expression.chars().nth(before_expression.len() - 2).unwrap();
        let before_value = evaluate_expression(before_expression.get(..before_expression.len() - 3).unwrap());

        return match operator {
            '+' => before_value + parens_value,
            '*' => before_value * parens_value,
            _ => panic!()
        }
    } else if ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&last_char) {
        let pieces: Vec<&str> = expression.rsplitn(3, ' ').collect();
        let last_value = pieces[0].parse::<u64>().unwrap();
        if pieces.len() == 1 {
            return last_value;
        }
        assert_eq!(pieces.len(), 3);
        assert_eq!(pieces[1].len(), 1);
        let operator = pieces[1].chars().nth(0).unwrap();

        let rest_value = evaluate_expression(pieces[2]);

        return match operator {
            '+' => rest_value + last_value,
            '*' => rest_value * last_value,
            _ => panic!()
        }
    } else {
        panic!();
    } 
}

//    Part 2
// ------------

fn find(v: &Vec<char>, c: char) -> usize {
    v.iter().position(|&x| x == c).unwrap()
}

/// Convert an expression in infix notation to Reverse Polish Notation
/// Operators earlier in `operator_precedence` have greater precedence.
fn infix_expression_to_rpn(infix_expression: &str, operator_precedence: &Vec<char>) -> Box<String> {
    // Put spaces after parens so we can tokenize on spaces.
    let infix_expression_spaced = infix_expression.replace("(", "( ").replace(")", " )");

    let mut rpn_expression: String = "".to_string();
    let mut operator_stack = Vec::<char>::new();
    for token in infix_expression_spaced.trim().split_ascii_whitespace() {
        if let Ok(_number) = token.parse::<u64>() {
            // Token is a number.
            rpn_expression.push_str(token);
            rpn_expression.push(' ');
        } else {
            assert_eq!(token.len(), 1);
            let token_char = token.chars().nth(0).unwrap();
            if token_char == '(' {
                operator_stack.push(token_char);
            } else if token_char == ')' {
                while *operator_stack.last().unwrap() != '(' {  // empty operator_stack here means mismatched parentheses.
                    rpn_expression.push(operator_stack.pop().unwrap());
                    rpn_expression.push(' ');
                }
                if *operator_stack.last().unwrap() == '(' {
                    operator_stack.pop();
                }
            } else if operator_precedence.contains(&token_char) {
                // Token is an operator.
                while !operator_stack.is_empty()
                    && *operator_stack.last().unwrap() != '('
                    && find(&operator_precedence, *operator_stack.last().unwrap())
                        <= find(&operator_precedence, token_char)
                {
                    rpn_expression.push(operator_stack.pop().unwrap());
                    rpn_expression.push(' ');
                }
                operator_stack.push(token_char);
            }
        }
    }
    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        if op == '(' || op == ')' { panic!(); } // Mismatched parentheses.
        rpn_expression.push(op);
        rpn_expression.push(' ');
    }

    Box::new(rpn_expression.trim().to_string())
}

/// Evaluate a Reverse Polish Notation expression.
fn evaluate_rpn_expression(rpn_expression: &str) -> i64 {
    let mut stack = Vec::<i64>::new();
    for token in rpn_expression.trim().split_ascii_whitespace() {
        if let Ok(number) = token.parse::<i64>() {
            stack.push(number);
        } else {
            assert_eq!(token.len(), 1);
            let token_char = token.chars().nth(0).unwrap();
            let number2 = stack.pop().unwrap();
            let number1 = stack.pop().unwrap();
            stack.push(
                match token_char {
                    '+' => number1 + number2,
                    '-' => number1 - number2,
                    '*' => number1 * number2,
                    '/' => number1 / number2,
                    _ => panic!()
                }
            );
        }
    }
    stack.pop().unwrap()
}

/// Evaluate an expression in infix notation, using a provided operator precedence.
fn evaluate_infix_expression(infix_expression: &str, operator_precedence: &Vec<char>) -> i64 {
    let rpn_expression = *infix_expression_to_rpn(infix_expression, operator_precedence);
    evaluate_rpn_expression(&rpn_expression)
}



#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest(infix, correct_rpn,
        case("3 + 4", "3 4 +"),
        case("3 + 4 * (2 - 1)", "3 4 2 1 - * +"),
    )]
    fn test_infix_expression_to_rpn(infix: &str, correct_rpn: &str) {
        let operator_precedence = vec!['*', '/', '+', '-']; // Normal math.
        assert_eq!(*infix_expression_to_rpn(infix, &operator_precedence), correct_rpn);
    }

    #[rstest(rpn_expression, correct_value,
        case("3 4 +", 7),
        case("3 4 5 * -", -17),
    )]
    fn test_evaluate_rpn_expression(rpn_expression: &str, correct_value: i64) {
        assert_eq!(evaluate_rpn_expression(rpn_expression), correct_value);
    }

    #[rstest(
        expression, correct_value,
        case("1 + 2 * 3 + 4 * 5 + 6", 71),
        case("1 + (2 * 3) + (4 * (5 + 6))", 51),
        case("2 * 3 + (4 * 5)", 26),
        case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    )]
    fn test_examples_part1(expression: &str, correct_value: u64) {
        assert_eq!(evaluate_expression(&expression), correct_value);
    }
    
    #[rstest(
        expression, correct_value,
        case("1 + 2 * 3 + 4 * 5 + 6", 231),
        case("1 + (2 * 3) + (4 * (5 + 6))", 51),
        case("2 * 3 + (4 * 5)", 46),
        case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    )]
    fn test_examples_part2(expression: &str, correct_value: i64) {
        let operator_precedence = vec!['+', '*']; // Part 2's operator precedence.
        assert_eq!(evaluate_infix_expression(&expression, &operator_precedence), correct_value);
    }

}