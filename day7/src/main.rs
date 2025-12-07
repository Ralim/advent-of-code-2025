use array2d::Array2D;
use memoize::memoize;
use shared::{
    ArrayPosition, ChallengeDay, Directions, Question, get_question_data_to_grid,
    move_grid_cursor_by_direction, print_array,
};
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
    println!("Starting Part A");
    let mut input_file = get_question_data_to_grid(ChallengeDay::Day7, question);

    // Now walk each row expanding out the beams.
    let mut num_beam_splits = 0;
    for row in 1..input_file.num_rows() {
        // Look at the square above to decide what to do.
        for col in 0..input_file.num_columns() {
            let above = *input_file.get(row - 1, col).unwrap();
            let current = *input_file.get(row, col).unwrap();
            if above == b'S' && current == b'.' {
                input_file.set(row, col, b'|').unwrap();
            }
            if above == b'|' && current == b'.' {
                input_file.set(row, col, b'|').unwrap();
            }

            if current == b'^' && above == b'|' {
                num_beam_splits += 1;

                // Set square to left and right to a beam if blank space
                let left = move_grid_cursor_by_direction(&input_file, Directions::LEFT, (row, col));
                let right =
                    move_grid_cursor_by_direction(&input_file, Directions::RIGHT, (row, col));
                if let Some(new_pos) = left {
                    // If spot at new_pos is a '.' set it to a beam
                    if input_file.get(new_pos.0, new_pos.1).unwrap() == &b'.' {
                        input_file.set(new_pos.0, new_pos.1, b'|').unwrap();
                    }
                }

                if let Some(new_pos) = right {
                    // If spot at new_pos is a '.' set it to a beam
                    if input_file.get(new_pos.0, new_pos.1).unwrap() == &b'.' {
                        input_file.set(new_pos.0, new_pos.1, b'|').unwrap();
                    }
                }
            }
        }
    }
    print_array(&input_file);

    num_beam_splits
}

#[memoize(Ignore: input_file)]
fn recursively_explore_grid(
    input_file: &Array2D<u8>,
    beam_at: ArrayPosition,
    counter: usize,
) -> usize {
    let row = beam_at.0 + 1;
    if row == input_file.num_rows() {
        return 1;
    }
    // Walk across the row, copy rays down and fork on splitters

    let current = *input_file.get(row, beam_at.1).unwrap();

    if current == b'.' {
        return recursively_explore_grid(input_file, (row, beam_at.1), counter);
    }
    if current == b'^' {
        return recursively_explore_splitter(input_file, (row, beam_at.1), counter);
    }
    0
}
fn recursively_explore_splitter(
    input_file: &Array2D<u8>,
    splitter_pos: ArrayPosition,
    split_counter: usize,
) -> usize {
    // We are at splitter at splitter_pos. Draw in the before and after lines
    let left = move_grid_cursor_by_direction(input_file, Directions::LEFT, splitter_pos);
    let right = move_grid_cursor_by_direction(input_file, Directions::RIGHT, splitter_pos);
    let mut count = 0;
    if let Some(new_pos) = left {
        let pos = input_file.get(new_pos.0, new_pos.1).unwrap();
        if pos == &b'.' {
            count += recursively_explore_grid(input_file, new_pos, split_counter);
        }
    }
    if let Some(new_pos) = right {
        let pos = input_file.get(new_pos.0, new_pos.1).unwrap();
        if pos == &b'.' {
            count += recursively_explore_grid(input_file, new_pos, split_counter);
        }
    }
    count
}

fn part_b(question: Question) -> usize {
    println!("Starting Part B");
    let input_file = get_question_data_to_grid(ChallengeDay::Day7, question);
    let start_col = (0..input_file.num_columns())
        .find(|&col| input_file.get(0, col) == Some(&b'S'))
        .unwrap();
    recursively_explore_grid(&input_file, (0, start_col), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 21);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 40);
    }
}
