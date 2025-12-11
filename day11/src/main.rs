use graphrs::Edge;
use memoize::memoize;
use shared::{ChallengeDay, Question, file_lines_to_graph_directed, get_question_data_lines};
use std::collections::{HashMap, VecDeque};
fn main() {
    let t_a = std::thread::spawn(|| {
        let ans = part_a(Question::Question);
        println!("Part A:{ans}");
    });
    let t_b = std::thread::spawn(|| {
        let ans = part_b(Question::Question);
        println!("Part B:{ans}");
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}

fn part_a(question: Question) -> u32 {
    let input_file = get_question_data_lines(ChallengeDay::Day11, question);
    // Each line is device: {connections space seperated}
    let graph = file_lines_to_graph_directed(&input_file, |line| {
        let (node, links) = line.split_once(':').unwrap();
        let linked_nodes = links.split_whitespace().map(|s| s.to_string());

        linked_nodes
            .into_iter()
            .map(|n| Edge::new(node.to_owned(), n))
            .collect()
    });
    // Find all paths from the node "you" to the node "out"
    // Using bfs
    let mut path_counter = 0;
    let mut todo = VecDeque::new();
    todo.push_back(("you".to_string(), vec![]));

    while let Some((node, path)) = todo.pop_front() {
        if node == "out" {
            path_counter += 1;
            continue;
        }
        let next_nodes = graph.get_successor_nodes(node.clone()).unwrap();
        // println!("Next nodes: {:?} from {:?}", next_nodes, node);
        for next_node in next_nodes {
            let mut next_path = path.clone();
            next_path.push(node.clone());
            todo.push_back((next_node.name.clone(), next_path));
        }
    }

    path_counter
}

fn part_b(question: Question) -> usize {
    let input_file = get_question_data_lines(ChallengeDay::Day11, question);

    // Each line is device: {connections space seperated}
    let graph = file_lines_to_graph_directed(&input_file, |line| {
        let (node, links) = line.split_once(':').unwrap();
        let linked_nodes = links.split_whitespace().map(|s| s.to_string());

        linked_nodes
            .into_iter()
            .map(|n| Edge::new(node.to_owned(), n))
            .collect()
    });

    // Pre-compute adjacency map for efficient memoization
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in graph.get_all_edges() {
        adjacency
            .entry(edge.u.clone())
            .or_default()
            .push(edge.v.clone());
    }

    // Convert to Vec once for memoization efficiency
    let adjacency_vec: Vec<(String, Vec<String>)> = adjacency
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // DFS with memoization using the memoize crate

    dfs_memoized(
        adjacency_vec,
        "svr".to_string(),
        "out".to_string(),
        false,
        false,
    )
}

// Memoized helper function for DFS without cycle detection
#[memoize]
fn dfs_memoized(
    adjacency: Vec<(String, Vec<String>)>,
    current: String,
    target: String,
    has_dac: bool,
    has_fft: bool,
) -> usize {
    // If we've reached the target, return 1 if we've seen both dac and fft
    if current == target {
        return if has_dac && has_fft { 1 } else { 0 };
    }

    // Update dac/fft flags based on current node
    let new_has_dac = has_dac || current == "dac";
    let new_has_fft = has_fft || current == "fft";

    let mut total_paths = 0;

    // Get neighbors from adjacency list
    for (node, neighbors) in &adjacency {
        if node == &current {
            for neighbor in neighbors {
                total_paths += dfs_memoized(
                    adjacency.clone(),
                    neighbor.clone(),
                    target.clone(),
                    new_has_dac,
                    new_has_fft,
                );
            }
            break;
        }
    }

    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::AltSample), 5);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 2);
    }
}
