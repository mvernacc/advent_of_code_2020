#[macro_use]
extern crate lazy_static;

use std::fs;
use petgraph::graphmap::DiGraphMap;
use petgraph::dot::{Dot, Config};
use regex::Regex;


fn main() {
    let text = fs::read_to_string("./example_input.txt").unwrap();
    let graph = parse_graph_from_text(&text);
    println!("{:?}", Dot::new(&graph));
}

fn parse_graph_from_text(text: &str) -> DiGraphMap::<&str, u32> {
    let lines = text
        .trim()
        .split("\n")
        .collect::<Vec<&str>>();
    let mut graph = DiGraphMap::<&str, u32>::new();
    for rule_str in lines {
        graph = add_rule_to_graph(graph, &rule_str);
    }
    graph
}

fn add_rule_to_graph<'a, 'b>(mut graph: DiGraphMap<&'a str, u32>, rule_str: &'a str) -> DiGraphMap::<&'a str, u32> {
    let (subject_bag_type, contained) = parse_rule(&rule_str);

    if !graph.contains_node(subject_bag_type) {
        graph.add_node(subject_bag_type);
    }

    for (other_bag_type, number_in_subject_bag) in contained {
        if !graph.contains_node(&other_bag_type) {
            graph.add_node(other_bag_type);
        }
        graph.add_edge(subject_bag_type, other_bag_type, number_in_subject_bag);
    }
    graph
}

fn parse_rule(rule_str: &str) -> (&str, Vec::<(&str, u32)>) {
    lazy_static! {
        static ref RE_SUBJECT: Regex = Regex::new(r"(\w+ \w+) bags contain").unwrap();
        static ref RE_CONTAINS: Regex = Regex::new(r"(\d) (\w+ \w+) bags?[,.]").unwrap();

    }
    let cap_subject = RE_SUBJECT.captures(rule_str).unwrap();
    let subject_bag_type = cap_subject.get(1).unwrap().as_str();

    let mut contains = Vec::<(&str, u32)>::new();
    for cap in RE_CONTAINS.captures_iter(rule_str) {
        contains.push((
            // Color of the contained bag type.
            cap.get(2).unwrap().as_str(),
            // Number of this bag type which are contained in the subject bag type.
            cap[1].parse::<u32>().unwrap()
        ));
    }

    (subject_bag_type, contains)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_1 () {
        let rule_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let (subject_bag_type, contains) = parse_rule(rule_str);
        assert_eq!("light red", subject_bag_type);
        assert_eq!(2, contains.len());
        assert_eq!("bright white", contains[0].0);
        assert_eq!(1, contains[0].1);
    }
}