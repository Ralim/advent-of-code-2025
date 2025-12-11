use graphrs::{Edge, Graph, GraphSpecs};
use std::sync::Arc;

pub fn file_lines_to_graph<F>(lines: &Vec<&str>, line_mapper: F) -> Graph<String, ()>
where
    F: Fn(&str) -> Vec<Arc<Edge<String, ()>>>,
{
    let graph_edges: Vec<Arc<Edge<String, ()>>> =
        lines.iter().flat_map(|line| line_mapper(line)).collect();

    let mut graph: Graph<String, ()> = Graph::new(GraphSpecs::undirected_create_missing());

    graph.add_edges(graph_edges).unwrap();

    graph
}
pub fn file_lines_to_graph_directed<F>(lines: &Vec<String>, line_mapper: F) -> Graph<String, ()>
where
    F: Fn(&str) -> Vec<Arc<Edge<String, ()>>>,
{
    let graph_edges: Vec<Arc<Edge<String, ()>>> =
        lines.iter().flat_map(|line| line_mapper(line)).collect();

    let mut graph: Graph<String, ()> = Graph::new(GraphSpecs::directed_create_missing());

    graph.add_edges(graph_edges).unwrap();

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_lines() {
        let lines: Vec<&str> = vec![];
        let mapper = |_line: &str| vec![];
        let graph = file_lines_to_graph(&lines, mapper);
        assert_eq!(graph.get_all_nodes().len(), 0);
        assert_eq!(graph.get_all_edges().len(), 0);
    }

    #[test]
    fn test_multiple_edges() {
        let lines = vec!["a-b", "b-c", "c-d"];
        let mapper = |line: &str| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                vec![Edge::new(parts[0].to_owned(), parts[1].to_owned())]
            } else {
                vec![]
            }
        };
        let graph = file_lines_to_graph(&lines, mapper);
        assert_eq!(graph.get_all_nodes().len(), 4);
        assert_eq!(graph.get_all_edges().len(), 3);
    }
}
