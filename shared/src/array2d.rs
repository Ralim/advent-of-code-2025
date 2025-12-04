use array2d::Array2D;

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
    point: (usize, usize),
    adjacents: Adjacents,
) -> Vec<(usize, usize)> {
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
}
