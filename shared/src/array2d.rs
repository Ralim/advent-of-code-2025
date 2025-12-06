use array2d::Array2D;
pub type ArrayPosition = (usize, usize);

pub fn rotate_array(mut grid: Array2D<u8>) -> Array2D<u8> {
    //Rotate the array 90 deg to the right
    // Do this by transposing the array, then reversing the rows
    assert_eq!(grid.row_len(), grid.column_len());

    //Transpose
    for r in 0..grid.num_rows() {
        for c in 0..r {
            let tmp = *grid.get(r, c).unwrap();
            let tmp2 = *grid.get(c, r).unwrap();
            grid.set(r, c, tmp2).unwrap();
            grid.set(c, r, tmp).unwrap();
        }
    }
    // Flip each row horizontally
    for r in 0..grid.num_rows() {
        for c in 0..grid.num_columns() / 2 {
            let tmp = *grid.get(r, c).unwrap();
            grid.set(r, c, *grid.get(r, grid.num_columns() - c - 1).unwrap())
                .unwrap();
            grid.set(r, grid.num_columns() - c - 1, tmp).unwrap();
        }
    }
    grid
}
pub fn print_array(array: &Array2D<u8>) {
    println!();
    for row_iter in array.rows_iter() {
        for element in row_iter {
            print!("{}", str::from_utf8(&[*element]).unwrap());
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Directions {
    pub fn get_direction(&self) -> (i64, i64) {
        match self {
            Directions::UP => (-1, 0),
            Directions::DOWN => (1, 0),
            Directions::LEFT => (0, -1),
            Directions::RIGHT => (0, 1),
        }
    }
    pub fn all() -> [Directions; 4] {
        [
            Directions::UP,
            Directions::DOWN,
            Directions::LEFT,
            Directions::RIGHT,
        ]
    }
    pub fn opposite(&self) -> Directions {
        match self {
            Directions::UP => Directions::DOWN,
            Directions::DOWN => Directions::UP,
            Directions::LEFT => Directions::RIGHT,
            Directions::RIGHT => Directions::LEFT,
        }
    }
    pub fn clockwise(&self) -> Directions {
        match self {
            Directions::UP => Directions::RIGHT,
            Directions::DOWN => Directions::LEFT,
            Directions::LEFT => Directions::UP,
            Directions::RIGHT => Directions::DOWN,
        }
    }
    pub fn counterclockwise(&self) -> Directions {
        match self {
            Directions::UP => Directions::LEFT,
            Directions::DOWN => Directions::RIGHT,
            Directions::LEFT => Directions::DOWN,
            Directions::RIGHT => Directions::UP,
        }
    }
}
pub fn move_grid_cursor_by_direction(
    array: &Array2D<u8>,
    direction: Directions,
    cursor: ArrayPosition,
) -> Option<ArrayPosition> {
    let (dx, dy) = direction.get_direction();
    let x = cursor.0 as i64 + dx;
    let y = cursor.1 as i64 + dy;
    if x >= 0 && x < array.num_rows() as i64 && y >= 0 && y < array.num_columns() as i64 {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

//Given an array for dimensions, and a start and end position, returns the movement direction sequence to get from start to end
// Returns None if no path is found
pub fn array_movement_sequence(
    array: &Array2D<u8>,
    start_pos: ArrayPosition,
    end_pos: ArrayPosition,
    banned_position_char: u8,
) -> Option<Vec<Directions>> {
    // Find the optimal path from start to end, without stepping on any grid positions with the banned character
    let mut current_position = start_pos;
    let mut steps = Vec::with_capacity(array.num_rows() * array.num_columns() / 2);
    while current_position != end_pos {
        let mut best_direction = None;
        let mut best_distance = i64::MAX;
        for direction in Directions::all() {
            let next_position = move_grid_cursor_by_direction(array, direction, current_position);
            if let Some(next_position) = next_position {
                let distance = (next_position.0 as i64 - end_pos.0 as i64).abs()
                    + (next_position.1 as i64 - end_pos.1 as i64).abs();
                if distance < best_distance && array[next_position] != banned_position_char {
                    best_direction = Some(direction);
                    best_distance = distance;
                }
            }
        }
        if let Some(direction) = best_direction {
            steps.push(direction);
            current_position =
                move_grid_cursor_by_direction(array, direction, current_position).unwrap();
        } else {
            return None;
        }
    }
    Some(steps)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjacents {
    ALL,        //All below
    DIAGONAL,   // Diagonal corners
    HORIZONTAL, //Two either side in the same row
    VERTICAL,   // Tow either side vertically in the same column
    CROSS,      // HORIZONTAL+VERTICAL
}

impl Adjacents {
    pub fn get_pattern(&self) -> &[(i64, i64)] {
        match self {
            Adjacents::ALL => &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            Adjacents::DIAGONAL => &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
            Adjacents::HORIZONTAL => &[(0, -1), (0, 1)],
            Adjacents::VERTICAL => &[(-1, 0), (1, 0)],
            Adjacents::CROSS => &[(-1, 0), (0, -1), (0, 1), (1, 0)],
        }
    }
}
pub fn adjacent_positions(
    grid: &Array2D<u8>,
    point: ArrayPosition,
    adjacents: Adjacents,
) -> Vec<ArrayPosition> {
    let (row, col) = point;
    let mut positions = Vec::new();

    let directions = adjacents.get_pattern();

    for (dr, dc) in directions {
        let new_row = row as i64 + dr;
        let new_col = col as i64 + dc;

        if new_row >= 0 && new_col >= 0 {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if new_row < grid.num_rows() && new_col < grid.num_columns() {
                positions.push((new_row, new_col));
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_positions_all() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (1, 1), Adjacents::ALL);
        let mut expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            // Skip itself
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_adjacent_positions_cross() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (1, 1), Adjacents::CROSS);
        let mut expected = vec![(0, 1), (1, 0), (1, 2), (2, 1)];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_adjacent_positions_diagonal() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (1, 1), Adjacents::DIAGONAL);
        let mut expected = vec![(0, 0), (0, 2), (2, 0), (2, 2)];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_adjacent_positions_horizontal() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (1, 1), Adjacents::HORIZONTAL);
        let mut expected = vec![(1, 0), (1, 2)];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_adjacent_positions_vertical() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (1, 1), Adjacents::VERTICAL);
        let mut expected = vec![(0, 1), (2, 1)];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_adjacent_positions_corner() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (0, 0), Adjacents::ALL);
        let mut expected = vec![(0, 1), (1, 0), (1, 1)];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }

    #[test]
    fn test_adjacent_positions_edge() {
        let grid = Array2D::filled_with(0u8, 3, 3);
        let result = adjacent_positions(&grid, (0, 1), Adjacents::ALL);
        let mut expected = vec![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)];
        let mut sorted_result = result.clone();
        sorted_result.sort();
        expected.sort();
        assert_eq!(sorted_result, expected);
    }
    #[test]
    fn test_array_movement_sequence_simple_path() {
        let grid = Array2D::filled_with(b'.', 5, 5);
        let start = (0, 0);
        let end = (2, 2);
        let result = array_movement_sequence(&grid, start, end, b'#').unwrap();

        // Should move down 2 and right 2
        assert_eq!(result.len(), 4);

        // Verify we can reach the end following the path
        let mut current = start;
        for direction in result {
            current = move_grid_cursor_by_direction(&grid, direction, current).unwrap();
        }
        assert_eq!(current, end);
    }

    #[test]
    fn test_array_movement_sequence_with_obstacles() {
        let mut grid = Array2D::filled_with(b'.', 3, 3);
        // Place obstacle at (1, 1)
        grid.set(1, 1, b'#').unwrap();

        let start = (0, 0);
        let end = (2, 2);
        let result = array_movement_sequence(&grid, start, end, b'#').unwrap();

        // Verify path avoids the obstacle
        let mut current = start;
        for direction in result {
            current = move_grid_cursor_by_direction(&grid, direction, current).unwrap();
            assert_ne!(grid[current], b'#'); // Should never step on obstacle
        }
        assert_eq!(current, end);
    }

    #[test]
    fn test_array_movement_sequence_same_position() {
        let grid = Array2D::filled_with(b'.', 3, 3);
        let pos = (1, 1);
        let result = array_movement_sequence(&grid, pos, pos, b'#').unwrap();

        // No movement needed when start equals end
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_array_movement_sequence_blocked_path() {
        let mut grid = Array2D::filled_with(b'#', 3, 3);
        // Only start position is free
        grid.set(0, 0, b'.').unwrap();

        let start = (0, 0);
        let end = (2, 2);
        let result = array_movement_sequence(&grid, start, end, b'#');
        assert_eq!(result, None);
    }

    #[test]
    fn test_array_movement_sequence_straight_line() {
        let grid = Array2D::filled_with(b'.', 1, 5);
        let start = (0, 0);
        let end = (0, 4);
        let result = array_movement_sequence(&grid, start, end, b'#').unwrap();

        // Should be 4 RIGHT moves
        assert_eq!(result.len(), 4);
        for direction in result {
            assert_eq!(direction, Directions::RIGHT);
        }
    }
    #[test]
    fn test_directions_get_direction() {
        assert_eq!(Directions::UP.get_direction(), (-1, 0));
        assert_eq!(Directions::DOWN.get_direction(), (1, 0));
        assert_eq!(Directions::LEFT.get_direction(), (0, -1));
        assert_eq!(Directions::RIGHT.get_direction(), (0, 1));
    }

    #[test]
    fn test_directions_all() {
        let all_directions = Directions::all();
        assert_eq!(all_directions.len(), 4);
        assert!(all_directions.contains(&Directions::UP));
        assert!(all_directions.contains(&Directions::DOWN));
        assert!(all_directions.contains(&Directions::LEFT));
        assert!(all_directions.contains(&Directions::RIGHT));
    }

    #[test]
    fn test_directions_opposite() {
        assert_eq!(Directions::UP.opposite(), Directions::DOWN);
        assert_eq!(Directions::DOWN.opposite(), Directions::UP);
        assert_eq!(Directions::LEFT.opposite(), Directions::RIGHT);
        assert_eq!(Directions::RIGHT.opposite(), Directions::LEFT);
    }

    #[test]
    fn test_directions_clockwise() {
        assert_eq!(Directions::UP.clockwise(), Directions::RIGHT);
        assert_eq!(Directions::RIGHT.clockwise(), Directions::DOWN);
        assert_eq!(Directions::DOWN.clockwise(), Directions::LEFT);
        assert_eq!(Directions::LEFT.clockwise(), Directions::UP);
    }

    #[test]
    fn test_directions_counterclockwise() {
        assert_eq!(Directions::UP.counterclockwise(), Directions::LEFT);
        assert_eq!(Directions::LEFT.counterclockwise(), Directions::DOWN);
        assert_eq!(Directions::DOWN.counterclockwise(), Directions::RIGHT);
        assert_eq!(Directions::RIGHT.counterclockwise(), Directions::UP);
    }

    #[test]
    fn test_directions_clockwise_counterclockwise_inverse() {
        for direction in Directions::all() {
            assert_eq!(direction.clockwise().counterclockwise(), direction);
            assert_eq!(direction.counterclockwise().clockwise(), direction);
        }
    }

    #[test]
    fn test_directions_opposite_inverse() {
        for direction in Directions::all() {
            assert_eq!(direction.opposite().opposite(), direction);
        }
    }

    #[test]
    fn test_move_grid_cursor_by_direction() {
        let grid = Array2D::filled_with(0u8, 5, 5);

        // Test valid moves from center
        let center = (2, 2);
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::UP, center),
            Some((1, 2))
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::DOWN, center),
            Some((3, 2))
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::LEFT, center),
            Some((2, 1))
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::RIGHT, center),
            Some((2, 3))
        );

        // Test boundary conditions - top-left corner
        let top_left = (0, 0);
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::UP, top_left),
            None
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::LEFT, top_left),
            None
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::DOWN, top_left),
            Some((1, 0))
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::RIGHT, top_left),
            Some((0, 1))
        );

        // Test boundary conditions - bottom-right corner
        let bottom_right = (4, 4);
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::DOWN, bottom_right),
            None
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::RIGHT, bottom_right),
            None
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::UP, bottom_right),
            Some((3, 4))
        );
        assert_eq!(
            move_grid_cursor_by_direction(&grid, Directions::LEFT, bottom_right),
            Some((4, 3))
        );
    }
}
