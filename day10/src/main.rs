use std::{fs, convert::TryFrom, collections::HashMap};
use petgraph::{graphmap::DiGraphMap, Direction};


fn main() {
    let text = fs::read_to_string("./input.txt").unwrap();
    let adapters = text
        .trim()
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Part 1
    let difference_counts: [u32; 4] = *compute_jolt_differences(&adapters).unwrap();
    println!("Number of 1-jolt differences multiplied by the number of 3-jolt differences: {}",
        difference_counts[1] * difference_counts[3]);
    
    // Part 2
    let path_count = count_paths_outlet_to_device(&adapters);
    println!("Number of distinct ways you can arrange the adapters to connect the charging outlet to your device: {}",
        path_count);
}

fn compute_jolt_differences(adapters: &[u32]) -> Option<Box<[u32; 4]>> {
    // If `difference_counts[i] == k`, there are `k` joltage differences of `i` jolts
    // in the adapter chain.
    let mut difference_counts = [0, 0, 0, 0];
    let mut unused_adapters = adapters.to_vec();
    // The jolt rating of the built-in joltage adapter.
    let built_in_joltage = adapters.iter().max().unwrap() + 3;
    unused_adapters.push(built_in_joltage);

    unused_adapters.sort();
    unused_adapters.reverse();

    // "The charging outlet has an effective rating of 0 jolts", so start our adapter chain
    // at 0 jolts.
    let mut jolt_level = 0;
    while !unused_adapters.is_empty() {
        let lowest_adapter = unused_adapters.pop().unwrap();
        let difference = lowest_adapter - jolt_level;
        if difference < 1 {
            println!("Lowest adapter {} too low for jolt level {}.", lowest_adapter, jolt_level);
            return None;
        }
        if difference > 3 {
            println!("Lowest adapter {} too high for jolt level {}.", lowest_adapter, jolt_level);
            return None;
        }
        difference_counts[usize::try_from(difference).unwrap()] += 1;
        jolt_level = lowest_adapter;
    }

    return Some(Box::new(difference_counts));
}

/// Builds a (directed acyclic) graph of which adapter joltage levels can connect to each other.
/// The returned graph contains a node for each adapter joltage level, and a `0`
/// node for the charging outlet, and a node for the device's built-in adapter.
/// There is an edge from node `i` to node `j` IFF adapter `j` can connect after adapter
/// `i`, i.e. `j - i` equals 1, 2 or 3. 
fn build_graph(adapters: &[u32]) -> Box<DiGraphMap<u32, ()>> {
    let mut joltage_levels = adapters.to_vec();
    joltage_levels.sort();
    joltage_levels.push(joltage_levels.last().unwrap() + 3);
    joltage_levels.insert(0, 0);

    let mut graph = DiGraphMap::<u32, ()>::new();
    for x in joltage_levels.iter() {
        graph.add_node(*x);
    }
    for x in joltage_levels.iter() {
        for difference in &[1, 2, 3] {
            if graph.contains_node(x + difference) {
                graph.add_edge(*x, x + difference, ());
            }
        }
    }

    return Box::new(graph);
}

/// Count the paths by which the 0-joltage outlet can connect to the device.
fn count_paths_outlet_to_device(adapters: &[u32]) -> u64 {
    let graph = *build_graph(&adapters);

    let mut joltage_levels = graph.nodes().collect::<Vec<u32>>();
    joltage_levels.sort();
    joltage_levels.reverse(); // Put the highest joltage level first
    assert_eq!(*joltage_levels.last().unwrap(), 0);

    // Node at joltage level `i` has `paths_to_end[i]` paths to reach the end node.
    // This is the memoization data structure for the dynamic programming solution.
    let mut paths_to_end = HashMap::<u32, u64>::new();
    paths_to_end.insert(joltage_levels[0], 1);

    // Count the number of paths from the `0` node (i.e. `joltage_levels.last()`)
    // to the "builtin joltage adapter" node (i.e. `joltage_levels[0]`)
    // using dynamic programming.
    for j in &joltage_levels[1..] {
        let mut paths_to_end_this_node = 0;
        // Each `connectable_joltage` is an (higher) adapter joltage level that can connect to
        // joltage level `j`. 
        for connectable_joltage in graph.neighbors_directed(*j, Direction::Outgoing) {
            // Note that `paths_to_end` must already have a key for `connectable_joltage`.
            // Only joltages higher than `j` are connectable, and we are walking through the
            // joltage levels from highest to lowest, so the higher levels have already been
            // processed. 
            paths_to_end_this_node += paths_to_end[&connectable_joltage];
        }
        paths_to_end.insert(*j, paths_to_end_this_node);
    }

    return paths_to_end[&0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jolt_differences_small_example() {
        // Setup
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        // Action
        let difference_counts: [u32; 4] = *compute_jolt_differences(&adapters).unwrap();
        // Verification
        assert_eq!(difference_counts[1], 7);
        assert_eq!(difference_counts[2], 0);
        assert_eq!(difference_counts[3], 5);
    }

    #[test]
    fn test_jolt_differences_larger_example() {
        // Setup
        let adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45,
            19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        // Action
        let difference_counts: [u32; 4] = *compute_jolt_differences(&adapters).unwrap();
        // Verification
        assert_eq!(difference_counts[1], 22);
        assert_eq!(difference_counts[2], 0);
        assert_eq!(difference_counts[3], 10);
    }

    #[test]
    fn test_count_paths_small_example() {
        // Setup
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        // Action
        let path_count = count_paths_outlet_to_device(&adapters);
        // Verification
        assert_eq!(path_count, 8);
    }

    #[test]
    fn test_count_paths_larger_example() {
        // Setup
        let adapters = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45,
            19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        // Action
        let path_count = count_paths_outlet_to_device(&adapters);
        // Verification
        assert_eq!(path_count, 19208);
    }
}