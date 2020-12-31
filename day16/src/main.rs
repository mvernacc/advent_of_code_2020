use std::{ fs, collections::{ HashMap, HashSet } };

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let error_rate = solve_part_1(&text);
    println!("Part 1: {}", error_rate);

    // Part 2.
    let your_ticket = *find_fields_your_ticket(&text);
    let mut product: u64 = 1;
    for (field_name, value) in your_ticket.iter() {
        if field_name.starts_with("departure") {
            product *= *value as u64;
        }
    }
    println!("Part 2: {}", product);
}


fn is_in_any_range(value: u32, ranges: &Vec<(u32, u32)>) -> bool {
    for &range in ranges.iter() {
        if range.0 <= value && value <= range.1 { return true; }
    }
    return false;
}

fn solve_part_1(text: &str) -> u32 {
    let paragraphs = text.split("\n\n").collect::<Vec<&str>>();
    let fields_text = paragraphs[0];

    let mut ranges = Vec::<(u32, u32)>::new();
    for line in fields_text.split("\n") {
        let name_and_ranges = line.split(": ").collect::<Vec<&str>>();
        for range_str in name_and_ranges[1].split(" or ") {
            let numbers = range_str.split("-").collect::<Vec<&str>>();
            ranges.push((
                numbers[0].parse::<u32>().unwrap(),
                numbers[1].parse::<u32>().unwrap()
            ));
        }
    }

    let nearby_tickets_lines = paragraphs[2].trim().split("\n").collect::<Vec<&str>>();
    let mut nearby_tickets_values = Vec::<Vec<u32>>::new();
    for line in nearby_tickets_lines[1..].iter() {
        nearby_tickets_values.push(
            line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }

    let mut error_rate = 0;
    for ticket in nearby_tickets_values.iter() {
        for &value in ticket {
            if !is_in_any_range(value, &ranges) {
                error_rate += value;
            }
        }
    }
    error_rate
}

struct TicketField {
    name: String,
    ranges: Vec<(u32, u32)>,
}

impl TicketField {
    fn is_valid_value(&self, value: u32) -> bool {
        is_in_any_range(value, &self.ranges)
    }
}

fn str_to_ticket_field(s: &str) -> Box<TicketField> {
    let name_and_ranges = s.split(": ").collect::<Vec<&str>>();
    let name = name_and_ranges[0].to_string();

    let mut ranges = Vec::<(u32, u32)>::new();
    for range_str in name_and_ranges[1].split(" or ") {
        let numbers = range_str.split("-").collect::<Vec<&str>>();
        ranges.push((
            numbers[0].parse::<u32>().unwrap(),
            numbers[1].parse::<u32>().unwrap()
        ));
    }

    Box::new(TicketField { name, ranges })
}

fn purge_invalid_tickets(tickets: &Vec<Vec<u32>>, ranges: &Vec<(u32, u32)>) -> Box<Vec<Vec<u32>>> {
    let mut valid_tickets: Vec<Vec<u32>> = Vec::new();
    for ticket in tickets.iter() {
        if ticket.iter().all(|value| is_in_any_range(*value, &ranges)) {
            valid_tickets.push(ticket.to_vec());
        }
    }
    Box::new(valid_tickets)
}

fn find_fields_your_ticket(text: &str) -> Box<HashMap<String, u32>> {
    let paragraphs = text.split("\n\n").collect::<Vec<&str>>();

    // Parse the ticket fields.
    let fields_text = paragraphs[0];
    let mut fields_by_name = HashMap::<String, TicketField>::new();
    let mut all_ranges = Vec::<(u32, u32)>::new();
    for line in fields_text.split("\n") {
        let field = *str_to_ticket_field(&line);
        all_ranges.extend(field.ranges.iter().cloned());
        fields_by_name.insert(field.name.clone(), field);
    }

    // Parse your ticket's values.
    let your_tickets_lines = paragraphs[1].trim().split("\n").collect::<Vec<&str>>();
    assert_eq!(your_tickets_lines[0], "your ticket:");
    let your_ticket_values = your_tickets_lines[1]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Parse nearby tickets' values.
    let nearby_tickets_lines = paragraphs[2].trim().split("\n").collect::<Vec<&str>>();
    assert_eq!(nearby_tickets_lines[0], "nearby tickets:");
    let mut nearby_tickets_values = Vec::<Vec<u32>>::new();
    for line in nearby_tickets_lines[1..].iter() {
        nearby_tickets_values.push(
            line.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }

    let valid_nearby_tickets_values = *purge_invalid_tickets(&nearby_tickets_values, &all_ranges);

    // Narrow down which fields can possibly correspond to each ticket position.
    // Initially, all field names are possible for each ticket position.
    let mut possible_field_names_by_position = Vec::<HashSet<String>>::new();
    for _ in 0..your_ticket_values.len() {
        possible_field_names_by_position.push(
            fields_by_name.keys().cloned().collect::<HashSet<String>>());
    }
    // If, for a given ticket, a position's value is not valid for a field, remove that field from
    // being a possible match for that position.
    for ticket in valid_nearby_tickets_values.iter() {
        for (position, value) in ticket.iter().enumerate() {
            for (field_name, field) in fields_by_name.iter() {
                if !field.is_valid_value(*value) {
                    possible_field_names_by_position[position].remove(field_name);
                }
            }
        }
    }

    // If a position can only correspond to one field, remove that field from consideration
    // for all other positions. Keep doing this until we match all the positions & fields.
    assert!(possible_field_names_by_position.iter().any(|hs| hs.len() == 1));
    let mut position_to_field_name_matches = HashMap::<usize, String>::new();
    while position_to_field_name_matches.len() < your_ticket_values.len() {
        for position in 0..possible_field_names_by_position.len() {
            if possible_field_names_by_position[position].len() == 1 && !position_to_field_name_matches.contains_key(&position) {
                // Only one field name can correspond to ticket position `position`.
                let matched_position = position;
                let matched_name = possible_field_names_by_position[position].iter().next().unwrap().clone();
                position_to_field_name_matches.insert(matched_position, matched_name.clone());
                // Remove matched_name from all other sets in possible_field_names_by_position
                for position_other in 0..possible_field_names_by_position.len() {
                    if position_other != matched_position {
                        possible_field_names_by_position[position_other].remove(&matched_name);
                    }
                }
            }
        }
    }

    // Pack up the (field name -> value) map for your ticket, using the position-field matches found above.
    let mut your_ticket = HashMap::<String, u32>::new();
    for (position, value) in your_ticket_values.iter().enumerate() {
        your_ticket.insert(
            position_to_field_name_matches.get(&position).unwrap().to_string(),
            *value);
    }
    Box::new(your_ticket)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let text = fs::read_to_string("example_input.txt").unwrap();
        let error_rate = solve_part_1(&text);
        assert_eq!(error_rate, 71);
    }

    #[test]
    fn test_part_2_example() {
        let text = fs::read_to_string("example_input_2.txt").unwrap();
        let your_ticket = *find_fields_your_ticket(&text);
        assert_eq!(*your_ticket.get("class").unwrap(), 12);
        assert_eq!(*your_ticket.get("row").unwrap(), 11);
        assert_eq!(*your_ticket.get("seat").unwrap(), 13);
    }
}