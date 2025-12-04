use array2d::Array2D;
use shared::{Adjacents, ChallengeDay, Question, adjacent_positions, get_question_data_to_grid};

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
    let input_file = get_question_data_to_grid(ChallengeDay::Day4, question);
    // For every cell that has an `@` check if the surrounding 8 cells have less than 4 @'s
    find_valid_ones(&input_file).len() as u32
}

fn part_b(question: Question) -> u64 {
    println!("Starting Part B");
    let mut input_file = get_question_data_to_grid(ChallengeDay::Day4, question);
    // For every cell that has an `@` check if the surrounding 8 cells have less than 4 @'s
    let mut valid_count = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for pos in find_valid_ones(&input_file) {
            input_file.set(pos.0, pos.1, b'.').unwrap();
            valid_count += 1;
            removed = true;
        }
    }
    valid_count
}

fn find_valid_ones(input_file: &Array2D<u8>) -> Vec<(usize, usize)> {
    let mut valid_positions = Vec::new();
    for pos in input_file
        .enumerate_row_major()
        .filter_map(|(p, v)| if *v == b'@' { Some(p) } else { None })
    {
        let mut count = 0;
        for new_pos in adjacent_positions(input_file, pos, Adjacents::ALL) {
            if new_pos != pos && input_file.get(new_pos.0, new_pos.1).unwrap_or(&b' ') == &b'@' {
                count += 1;
            }
        }

        if count < 4 {
            valid_positions.push(pos);
        }
    }
    valid_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 13);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 43);
    }
}
