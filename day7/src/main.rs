#[macro_use]
extern crate lazy_static;

use std::{fs, collections::HashSet};
use petgraph::{graphmap::DiGraphMap, Direction};
use regex::Regex;


fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let graph = parse_graph_from_text(&text);
    let bag_color = "shiny gold";

    // Part 1.
    let bags_that_can_contain = compute_bags_that_can_contain(&graph, bag_color);
    println!("Number of bag colors which can eventually contain at least 1 '{:}' bag: {:}",
        bag_color, bags_that_can_contain.len());

    // Part 2.
    let num_bags_inside = compute_bags_inside(&graph, bag_color);
    println!("Number of bags required inside a '{:}' bag: {:}",
        bag_color, num_bags_inside);
}


/// Compute the number of bags inside a bag of a given color
fn compute_bags_inside(graph: &DiGraphMap<&str, u32>, bag_color: &str) -> u32 {
    assert!(graph.contains_node(bag_color));

    let mut count = 0;
    for v in graph.neighbors_directed(bag_color, Direction::Outgoing) {
        count += graph.edge_weight(bag_color, v).unwrap() * (1 + compute_bags_inside(graph, v));
    }
    count
}

/// Compute all the bag colors which can eventually contain a bag of color `start_bag`.
fn compute_bags_that_can_contain<'a>(graph: &DiGraphMap<&'a str, u32>, start_bag: &'a str) -> HashSet<&'a str> {
    assert!(graph.contains_node(start_bag));

    // Use breadth-first search.
    // Queue `Q` in CLRS page 595.
    let mut search_queue = Vec::<&str>::new();
    // Nodes which are colored black in CLRS page 595.
    let mut upstream_bags = HashSet::<&str>::new();
    search_queue.push(start_bag);
    while !search_queue.is_empty() {
        let u = search_queue.pop().unwrap();
        for v in graph.neighbors_directed(u, Direction::Incoming) {
            if !upstream_bags.contains(v) {
                search_queue.push(v);
            }
        }
        upstream_bags.insert(u);
    }

    upstream_bags.remove(start_bag);
    upstream_bags
}


fn parse_graph_from_text(text: &str) -> DiGraphMap<&str, u32> {
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
    let (subject_bag_color, contained) = parse_rule(&rule_str);

    if !graph.contains_node(subject_bag_color) {
        graph.add_node(subject_bag_color);
    }

    for (other_bag_color, number_in_subject_bag) in contained {
        if !graph.contains_node(&other_bag_color) {
            graph.add_node(other_bag_color);
        }
        graph.add_edge(subject_bag_color, other_bag_color, number_in_subject_bag);
    }
    graph
}

fn parse_rule(rule_str: &str) -> (&str, Vec::<(&str, u32)>) {
    lazy_static! {
        static ref RE_SUBJECT: Regex = Regex::new(r"(\w+ \w+) bags contain").unwrap();
        static ref RE_CONTAINS: Regex = Regex::new(r"(\d) (\w+ \w+) bags?[,.]").unwrap();

    }
    let cap_subject = RE_SUBJECT.captures(rule_str).unwrap();
    let subject_bag_color = cap_subject.get(1).unwrap().as_str();

    let mut contains = Vec::<(&str, u32)>::new();
    for cap in RE_CONTAINS.captures_iter(rule_str) {
        contains.push((
            // Color of the contained bag.
            cap.get(2).unwrap().as_str(),
            // Number of this bag type which are contained in the subject bag color.
            cap[1].parse::<u32>().unwrap()
        ));
    }

    (subject_bag_color, contains)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_1 () {
        let rule_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let (subject_bag_color, contains) = parse_rule(rule_str);
        assert_eq!("light red", subject_bag_color);
        assert_eq!(2, contains.len());
        assert_eq!("bright white", contains[0].0);
        assert_eq!(1, contains[0].1);
    }

    #[test]
    fn test_can_contain_example () {
        let text = fs::read_to_string("./example_input.txt").unwrap();
        let graph = parse_graph_from_text(&text);
        let containing_bags = compute_bags_that_can_contain(&graph, "shiny gold");
        assert_eq!(4, containing_bags.len());
        assert!(containing_bags.contains("bright white"));
        assert!(containing_bags.contains("muted yellow"));
        assert!(containing_bags.contains("dark orange"));
        assert!(containing_bags.contains("light red"));
    }
    
    #[test]
    fn compute_bags_inside_example1 () {
        let text = fs::read_to_string("./example_input.txt").unwrap();
        let graph = parse_graph_from_text(&text);

        assert_eq!(0, compute_bags_inside(&graph, "faded blue"));
        assert_eq!(0, compute_bags_inside(&graph, "dotted black"));
        assert_eq!(11, compute_bags_inside(&graph, "vibrant plum"));
        assert_eq!(7, compute_bags_inside(&graph, "dark olive"));
        assert_eq!(32, compute_bags_inside(&graph, "shiny gold"));

    }
}