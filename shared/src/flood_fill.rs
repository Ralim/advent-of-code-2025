use array2d::Array2D;

use crate::{Adjacents, ArrayPosition, adjacent_positions};

pub fn flood_fill(array_in: &mut Array2D<u8>, start: ArrayPosition, empty_char: u8, fill_with: u8) {
    // Starting at start, fill outwards all empty_char spots
    // creeps outwards in place to bound ram

    recursively_fill(array_in, start, empty_char, fill_with);
}
fn recursively_fill(
    array: &mut Array2D<u8>,
    start_position: ArrayPosition,
    empty_char: u8,
    fill_with: u8,
) {
    for spot in adjacent_positions(array, start_position, Adjacents::CROSS) {
        //All adjacent spots
        if array.get(spot.0, spot.1).unwrap() == &empty_char {
            array.set(spot.0, spot.1, fill_with).unwrap();
            recursively_fill(array, spot, empty_char, fill_with);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_positions_edge() {
        let mut grid = Array2D::from_row_major(
            &vec![
                b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'O', b'.', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'O', b'O', //
                b'.', b'O', b'.', b'O', b'O', b'.', b'.', b'O', b'.', b'O', //
                b'.', b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'O', b'.', //
                b'.', b'.', b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'.', //
                b'.', b'.', b'O', b'O', b'O', b'.', b'.', b'.', b'O', b'.', //
                b'.', b'.', b'O', b'.', b'O', b'O', b'.', b'.', b'O', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'O', b'.', b'.', b'O', b'.', //
                b'.', b'O', b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'.', //
                b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.', //
            ],
            10,
            10,
        )
        .unwrap();

        flood_fill(&mut grid, (9, 9), b'.', b'x');
        let expected = Array2D::from_row_major(
            &vec![
                b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'O', b'.', b'.', //
                b'x', b'O', b'O', b'x', b'x', b'x', b'x', b'O', b'O', b'O', //
                b'x', b'O', b'.', b'O', b'O', b'x', b'x', b'O', b'.', b'O', //
                b'x', b'O', b'.', b'.', b'.', b'O', b'O', b'.', b'O', b'x', //
                b'x', b'x', b'O', b'O', b'.', b'.', b'.', b'.', b'O', b'x', //
                b'x', b'x', b'O', b'O', b'O', b'.', b'.', b'.', b'O', b'x', //
                b'x', b'x', b'O', b'x', b'O', b'O', b'.', b'.', b'O', b'x', //
                b'x', b'O', b'O', b'x', b'x', b'O', b'.', b'.', b'O', b'x', //
                b'x', b'O', b'O', b'x', b'x', b'x', b'O', b'O', b'x', b'x', //
                b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x', //
            ],
            10,
            10,
        )
        .unwrap();
        assert_eq!(grid, expected);
    }
}
