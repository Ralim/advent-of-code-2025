use array2d::Array2D;
use rayon::prelude::*;
use shared::{
    ChallengeDay, Directions, Question, create_n_choice_sets_unordered, get_question_data_lines,
    move_grid_cursor_by_direction,
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

fn part_a(question: Question) -> i128 {
    println!("Starting Part A");
    let input_file = get_question_data_lines(ChallengeDay::Day9, question);
    let pairs: Vec<(i128, i128)> = input_file
        .into_iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let combos = create_n_choice_sets_unordered(&pairs, 2);
    // Find the largest combo set
    let mut max_size = 0;
    let mut max_pair = None;
    for pair in combos {
        let dx = (pair[0].0 - pair[1].0).abs() + 1;
        let dy = (pair[0].1 - pair[1].1).abs() + 1;
        let area = dx * dy;
        // println!("Pairs {pair:?} {area} {dx} {dy}");
        if area > max_size {
            max_size = area;
            max_pair = Some(pair);
        }
    }
    println!("Largest pair: {:?}", max_pair);
    //Return the area of the largest pair
    let max_pair = max_pair.unwrap();
    ((max_pair[0].0 - max_pair[1].0).abs() + 1) * ((max_pair[0].1 - max_pair[1].1).abs() + 1)
}

fn part_b(question: Question) -> i128 {
    println!("Starting Part B");
    let input_file = get_question_data_lines(ChallengeDay::Day9, question);
    let mut max_x = 0;
    let mut max_y = 0;
    let pairs: Vec<(i64, i64)> = input_file
        .into_iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            (y, x)
        })
        .collect();
    println!("Max X: {}, Max Y: {}", max_x, max_y);
    let mut array = Array2D::filled_with(b'.', max_y as usize + 2, max_x as usize + 2);
    // Join each pair N to pair N+1
    for (pair, next_pair) in pairs.iter().zip(pairs.iter().cycle().skip(1)) {
        array.set(pair.0 as usize, pair.1 as usize, b'#').unwrap();
        // Fill either row or column to next_pair
        if pair.0 == next_pair.0 {
            // Same row
            for x in pair.1.min(next_pair.1)..=pair.1.max(next_pair.1) {
                array.set(pair.0 as usize, x as usize, b'#').unwrap();
            }
        } else {
            for y in pair.0.min(next_pair.0)..=pair.0.max(next_pair.0) {
                array.set(y as usize, pair.1 as usize, b'#').unwrap();
            }
        }
        array
            .set(next_pair.0 as usize, next_pair.1 as usize, b'#')
            .unwrap();
    }
    // print_array(&array);
    println!("Filling in");

    for row in 0..array.num_rows() {
        let mut in_poly = false;
        for col in 0..array.num_columns() {
            // If this square is a # and the previous square is not a # then we are crossing an edge
            let previous_char = if let Some(previous) =
                move_grid_cursor_by_direction(&array, Directions::LEFT, (row, col))
            {
                *array.get(previous.0, previous.1).unwrap()
            } else {
                b'.'
            };
            let next_char = if let Some(next) =
                move_grid_cursor_by_direction(&array, Directions::RIGHT, (row, col))
            {
                *array.get(next.0, next.1).unwrap()
            } else {
                b'.'
            };

            // We are at an edge if previous chart is !#  or previous char
            if *array.get(row, col).unwrap() == b'#' {
                // This is an edge, we move left to right.
                // If the char to the left is a . then we are entering the polygon
                if previous_char == b'.' {
                    in_poly = true;
                } else if previous_char == b'#' {
                    // We are in a line
                    in_poly = true;
                } else if next_char == b'.' && in_poly {
                    // if *array.get(row, col).unwrap() == b'.' {
                    //     array.set(row, col, b'O').unwrap();
                    // }
                    in_poly = false;
                }
            } else if in_poly && *array.get(row, col).unwrap() == b'.' {
                array.set(row, col, b'O').unwrap();
            }
        }
    }
    // print_array(&array);
    println!("Find all combo lengths");
    let combos = create_n_choice_sets_unordered(&pairs, 2);

    let mut combo_size_enum: Vec<(usize, i64)> = combos
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(i, pair)| {
            let dx = (pair[0].0 - pair[1].0).abs() + 1;
            let dy = (pair[0].1 - pair[1].1).abs() + 1;
            let area = dx * dy;
            (i, area)
        })
        .collect();
    println!("Combo sizes done");
    // Sort combo_size enum by size decrementing
    combo_size_enum.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Combo sizes sorted");
    let len_combos = combo_size_enum.len();
    let mut num_processed = 0;
    for (i, _area) in combo_size_enum {
        num_processed += 1;
        let pair = &combos[i];
        if let Some(new_max) = get_pair_size_if_valid(&array, pair) {
            println!("Largest pair: {pair:?} {}", new_max);
            return new_max as i128;
        }
        if num_processed % 1000 == 0 {
            println!("Checked {} combos of {len_combos}", i);
        }
    }
    0
    //Return the area of the largest pair
}

fn get_pair_size_if_valid(array: &Array2D<u8>, pair: &[(i64, i64)]) -> Option<i64> {
    // println!("Pair check {pair:?} {area} {dx} {dy}");
    // Check that all squares in this rectangle are not a '.' in the array
    let row_min = pair[0].0.min(pair[1].0);
    let row_max = pair[0].0.max(pair[1].0);
    let col_min = pair[0].1.min(pair[1].1);
    let col_max = pair[0].1.max(pair[1].1);

    // First check all bounding edges
    for row in row_min..=row_max {
        if array.get(row as usize, col_min as usize).unwrap() == &b'.' {
            return None;
        }
        if array.get(row as usize, col_max as usize).unwrap() == &b'.' {
            return None;
        }
    }
    for col in col_min..=col_max {
        if array.get(row_min as usize, col as usize).unwrap() == &b'.' {
            return None;
        }
        if array.get(row_max as usize, col as usize).unwrap() == &b'.' {
            return None;
        }
    }
    for row in row_min + 1..row_max {
        for col in col_min + 1..col_max {
            if array.get(row as usize, col as usize).unwrap() == &b'.' {
                return None;
            }
        }
    }
    let dx = (pair[0].0 - pair[1].0).abs() + 1;
    let dy = (pair[0].1 - pair[1].1).abs() + 1;
    let area = dx * dy;
    Some(area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 50);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 24);
    }
}
