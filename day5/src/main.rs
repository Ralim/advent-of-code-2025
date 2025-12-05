use shared::{ChallengeDay, Question, get_question_data_lines};
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
    let input_file = get_question_data_lines(ChallengeDay::Day5, question);
    let mut fresh_ranges: Vec<(usize, usize)> = Vec::with_capacity(100);
    let mut ingredients: Vec<usize> = Vec::with_capacity(100);
    for line in input_file {
        if line.contains('-') {
            let parts = line.split_once('-');
            fresh_ranges.push(
                parts
                    .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                    .unwrap(),
            );
        } else if !line.is_empty() {
            ingredients.push(line.parse().unwrap())
        }
    }
    // Find freshies
    let mut fresh_ingredients: Vec<usize> = Vec::with_capacity(100);
    for ingredient in ingredients {
        for (start, end) in &fresh_ranges {
            if ingredient >= *start && ingredient <= *end {
                fresh_ingredients.push(ingredient);
                break;
            }
        }
    }
    fresh_ingredients.len() as u32
}

fn part_b(question: Question) -> usize {
    println!("Starting Part B");
    let input_file = get_question_data_lines(ChallengeDay::Day5, question);
    let mut fresh_ranges: Vec<(usize, usize)> = Vec::with_capacity(100);
    for line in input_file {
        if line.contains('-') {
            let parts = line.split_once('-');
            fresh_ranges.push(
                parts
                    .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                    .unwrap(),
            );
        }
    }
    // We now want to collapse down all the ranges that overlap with each other
    let mut de_overlapped_ranges: Vec<(usize, usize)> = Vec::with_capacity(fresh_ranges.len());

    // Sort fresh_ranges by start (.0)
    fresh_ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

    for range in fresh_ranges {
        let mut added = false;
        for existing_range in &mut de_overlapped_ranges {
            // Do we overlap either end of this range
            // This means the start or end overlaps the other range

            // Also check if the new range contains the existing range or vice versa
            let start = range.0;
            let end = range.1;
            let existing_start = existing_range.0;
            let existing_end = existing_range.1;

            let overlap_end = start >= existing_start && start <= existing_end;
            let overlap_start = end >= existing_start && end <= existing_end;
            if overlap_end || overlap_start {
                *existing_range = (range.0.min(existing_range.0), range.1.max(existing_range.1));
                added = true;
                // Break to outer loop
                break;
            }
        }
        if !added {
            de_overlapped_ranges.push(range);
        }
    }
    // Now find the total number in all ranges

    de_overlapped_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 3);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 14);
    }
}
