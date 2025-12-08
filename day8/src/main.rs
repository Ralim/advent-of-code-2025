use std::{collections::HashSet, sync::Arc};

use graphrs::{Edge, EdgeDedupeStrategy, Graph, GraphSpecs};
use shared::{ChallengeDay, Question, get_question_data_lines};
fn main() {
    let t_a = std::thread::spawn(|| {
        let ans = part_a(Question::Question, 1000);
        println!("Part A:{ans}");
    });
    let t_b = std::thread::spawn(|| {
        let ans = part_b(Question::Question);
        println!("Part B:{ans}");
    });
    t_a.join().unwrap();
    t_b.join().unwrap();
}

fn part_a(question: Question, n: usize) -> usize {
    println!("Starting Part A");
    let root_graph = load_question_graph(question);
    let mut edges = root_graph.get_all_edges();
    // Sort edges from lowest distance to highest distance
    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    // Make a new graph from just the first n edge
    let mut smaller_graph: Graph<String, ()> = Graph::new(GraphSpecs::undirected_create_missing());

    // Insert n edges
    edges.into_iter().take(n).for_each(|edge| {
        smaller_graph.add_edge((*edge).clone()).unwrap();
    });

    // Find all sub-graphs
    // We only have nodes that have a connection
    let all_nodes = smaller_graph.get_all_nodes();
    let mut sub_graphs = Vec::new();
    let mut nodes_seen = HashSet::with_capacity(all_nodes.len());

    for node in all_nodes {
        if !nodes_seen.contains(&node.name) {
            let nodes = smaller_graph.breadth_first_search(&node.name);
            nodes_seen.insert(node.name.to_owned());
            nodes_seen.extend(nodes.iter().cloned());
            sub_graphs.push(nodes);
        }
    }
    let mut circuit_sizes: Vec<usize> = sub_graphs.into_iter().map(|s| s.len()).collect();
    //Sort sizes large -> small
    circuit_sizes.sort();
    circuit_sizes.reverse();

    // Multiply the first 3 biggest
    circuit_sizes.iter().take(3).product()
}

fn load_question_graph(question: Question) -> Graph<String, ()> {
    let input_file = get_question_data_lines(ChallengeDay::Day8, question);
    let junction_boxes: Vec<(String, i64, i64, i64)> = input_file
        .into_iter()
        .map(|line| {
            // Split line on commas into 3
            let readings = line
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>();
            (line, readings[0], readings[1], readings[2])
        })
        .collect();

    // Connect the first 1000 pairs of junction boxes (those closest by distance)

    let mut graph_edges: Vec<Arc<Edge<String, ()>>> = vec![];

    // Add an edge to all other junction boxes, so we make it a fully connected graph
    for (index, junction_box) in junction_boxes.iter().enumerate() {
        for other_box in junction_boxes.iter().skip(index + 1) {
            let distance = ((junction_box.3 - other_box.3).pow(2)
                + (junction_box.1 - other_box.1).pow(2)
                + (junction_box.2 - other_box.2).pow(2)) as f64;
            let edge =
                Edge::with_weight(junction_box.0.clone(), other_box.0.clone(), distance.sqrt());
            graph_edges.push(edge);
        }
    }
    // Have it allow edge repeating
    let mut graph_specs = GraphSpecs::undirected_create_missing();
    graph_specs.edge_dedupe_strategy = EdgeDedupeStrategy::KeepFirst;
    let mut root_graph: Graph<String, ()> = Graph::new(graph_specs);
    root_graph.add_edges(graph_edges).unwrap();
    root_graph
}

fn part_b(question: Question) -> i32 {
    println!("Starting Part B");
    let root_graph = load_question_graph(question);
    let mut edges = root_graph.get_all_edges();
    // Sort edges from lowest distance to highest distance
    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    // Make a new graph from just the first n edge
    let mut smaller_graph: Graph<String, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    let total_junction_boxes = root_graph.number_of_nodes();
    println!("Total Junction Boxes {total_junction_boxes}");
    while !edges.is_empty() {
        let test_edge = edges.remove(0);
        smaller_graph.add_edge((*test_edge).clone()).unwrap();
        // Once all boxes exist, we can start checking for connectivity
        if smaller_graph.number_of_nodes() == total_junction_boxes {
            let first_node = smaller_graph.get_node_by_index(&0).unwrap();
            // Oh this is shit lol but eh, does give us the nice set
            let connected_nodes = smaller_graph.breadth_first_search(&first_node.name);

            if connected_nodes.len() == total_junction_boxes {
                let a = test_edge.u.split_once(',').unwrap();
                let b = test_edge.v.split_once(',').unwrap();
                let x1 = a.0.parse::<i32>().unwrap();
                let x2 = b.0.parse::<i32>().unwrap();
                println!("Found connector {} {}", x1, x2);
                return x1 * x2;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample, 10), 40);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 25272);
    }
}
