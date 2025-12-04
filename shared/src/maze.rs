use crate::{Adjacents, adjacent_positions};
use array2d::Array2D;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

/// Walk the maze from start to end using Dijkstra's algorithm. Avoiding walls and using a cost function for tile weights
pub fn maze_shortest_path<F>(
    maze: &Array2D<u8>,
    start: (usize, usize),
    end: (usize, usize),
    wall: u8,
    cost_fn: F,
) -> Vec<(usize, usize)>
where
    F: Fn(u8) -> u32,
{
    // Create a weighted graph using adjacency list
    // Each node maps to a list of (neighbor, weight) pairs
    let row_col_to_index = |(row, col)| -> u32 { (row * maze.num_columns() + col) as u32 };

    let graph: HashMap<u32, Vec<(u32, u32)>> = maze
        .enumerate_row_major()
        .filter(|(_, value)| *value != &wall)
        .map(|(pos, _value)| {
            // Check all of the surrounding positions
            let adjacent_positions = adjacent_positions(&maze, pos, Adjacents::CROSS);
            let mut edges = Vec::new();
            for adjacent_position in adjacent_positions {
                let adjacent_cell_value =
                    *maze.get(adjacent_position.0, adjacent_position.1).unwrap();
                if adjacent_cell_value != wall {
                    let cost = cost_fn(adjacent_cell_value);
                    edges.push((row_col_to_index(adjacent_position), cost));
                }
            }
            (row_col_to_index(pos), edges)
        })
        .collect();

    // Define the successor function
    let successors =
        |node: &u32| -> Vec<(u32, u32)> { graph.get(node).cloned().unwrap_or_default() };

    // Find shortest path from A to E using Dijkstra's algorithm
    let (path, _cost) = dijkstra(&row_col_to_index(start), successors, |&node| {
        node == row_col_to_index(end)
    })
    .unwrap();

    // Convert path from node indices back to (row, col) coordinates
    path.into_iter()
        .map(|node| {
            let row = (node as usize) / maze.num_columns();
            let col = (node as usize) % maze.num_columns();
            (row, col)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_shortest_path_10x10() {
        // Create a 10x10 maze with walls (1) and open spaces (0)
        // Layout:
        // . = open space (0)
        // # = wall (1)
        // S = start position
        // E = end position
        let maze_data = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // Row 0: all open
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0], // Row 1: walls with openings at ends
            vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 0], // Row 2: corridor
            vec![0, 1, 0, 1, 1, 1, 1, 0, 1, 0], // Row 3: inner walls
            vec![0, 1, 0, 1, 0, 0, 1, 0, 1, 0], // Row 4: maze structure
            vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 0], // Row 5: more maze
            vec![0, 1, 0, 0, 0, 1, 0, 0, 1, 0], // Row 6: path through
            vec![0, 1, 1, 1, 1, 1, 0, 1, 1, 0], // Row 7: walls
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0], // Row 8: path to end
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // Row 9: all open
        ];

        // Flatten the data for Array2D
        let flat_data: Vec<u8> = maze_data.into_iter().flatten().collect();
        let maze = Array2D::from_row_major(&flat_data, 10, 10).unwrap();

        let start = (0, 0); // Top-left corner
        let end = (9, 9); // Bottom-right corner
        let wall = 1u8;

        // Find the shortest path
        let path = maze_shortest_path(&maze, start, end, wall, |_| 10);

        // Verify the path
        assert!(!path.is_empty(), "Path should not be empty");
        assert_eq!(path[0], start, "Path should start at the start position");
        assert_eq!(
            path[path.len() - 1],
            end,
            "Path should end at the end position"
        );

        // Verify all positions in path are valid (not walls)
        for &(row, col) in &path {
            assert_ne!(
                *maze.get(row, col).unwrap(),
                wall,
                "Path should not go through walls at position ({}, {})",
                row,
                col
            );
        }

        // Verify path continuity (each step should be adjacent)
        for i in 1..path.len() {
            let (prev_row, prev_col) = path[i - 1];
            let (curr_row, curr_col) = path[i];
            let row_diff = (curr_row as i32 - prev_row as i32).abs();
            let col_diff = (curr_col as i32 - prev_col as i32).abs();

            assert!(
                (row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1),
                "Path steps should be adjacent: from ({}, {}) to ({}, {})",
                prev_row,
                prev_col,
                curr_row,
                curr_col
            );
        }

        assert!(
            path.len() < 20,
            "Path length should be reasonable, got {}",
            path.len()
        );
    }
}
