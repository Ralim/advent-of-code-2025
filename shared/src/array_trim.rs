use array2d::Array2D;

pub fn trim_array_to_bounds(array: Array2D<u8>, bg_char: u8) -> Array2D<u8> {
    // Find the lowest row,col and highest row,col that is not bg_char
    let min_row = (0..array.num_rows())
        .find(|row| (0..array.num_columns()).any(|col| array.get(*row, col).unwrap() != &bg_char))
        .unwrap();

    let max_row = (0..array.num_rows())
        .rev()
        .find(|row| (0..array.num_columns()).any(|col| array.get(*row, col).unwrap() != &bg_char))
        .unwrap();
    let min_col = (0..array.num_columns())
        .find(|col| (0..array.num_rows()).any(|row| array.get(row, *col).unwrap() != &bg_char))
        .unwrap();
    let max_col = (0..array.num_columns())
        .rev()
        .find(|col| (0..array.num_rows()).any(|row| array.get(row, *col).unwrap() != &bg_char))
        .unwrap();
    // Copy out the region from min<->max
    println!(
        " min_row: {min_row},    max_row: {max_row},    min_col: {min_col},    max_col: {max_col}"
    );
    let new_elements: Vec<u8> = array
        .enumerate_row_major()
        .filter_map(|((row, col), val)| {
            if row >= min_row && row <= max_row && col >= min_col && col <= max_col {
                Some(*val)
            } else {
                None
            }
        })
        .collect();
    Array2D::from_row_major(&new_elements, max_row - min_row + 1, max_col - min_col + 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_array_to_bounds() {
        let grid = Array2D::from_row_major(
            &vec![
                b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'O', b'O', b'.', b'.', //
                b'.', b'O', b'.', b'O', b'O', b'.', b'.', b'O', b'.', b'O', b'.', b'.', //
                b'.', b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'O', b'.', b'.', b'.', //
                b'.', b'.', b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'.', b'.', b'.', //
                b'.', b'.', b'O', b'O', b'O', b'.', b'.', b'.', b'O', b'.', b'.', b'.', //
                b'.', b'.', b'O', b'.', b'O', b'O', b'.', b'.', b'O', b'.', b'.', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'O', b'.', b'.', b'O', b'.', b'.', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'.', b'.', b'.', //
                b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', //
            ],
            10,
            12,
        )
        .unwrap();

        let res = trim_array_to_bounds(grid, b'.');
        let expected = Array2D::from_row_major(
            &vec![
                b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'O', b'O', //
                b'O', b'.', b'O', b'O', b'.', b'.', b'O', b'.', b'O', //
                b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'O', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'.', //
                b'.', b'O', b'O', b'O', b'.', b'.', b'.', b'O', b'.', //
                b'.', b'O', b'.', b'O', b'O', b'.', b'.', b'O', b'.', //
                b'O', b'O', b'.', b'.', b'O', b'.', b'.', b'O', b'.', //
                b'O', b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'.', //
            ],
            8,
            9,
        )
        .unwrap();
        assert_eq!(res, expected);
    }
}
