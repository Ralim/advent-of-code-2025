use iter_first_max::IterFirstMaxExt as _;
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
    let input_file = get_question_data_lines(ChallengeDay::Day3, question);
    // Each line is a series of numbers, we can turn on exactly two
    let mut sum = 0;
    for line in input_file {
        // Convert each character to an integer
        let numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        // Pick any two numbers, in order, and find the highest
        let index_of_highest_number = numbers
            .iter()
            .enumerate()
            .take(numbers.len() - 1) // Cant pick last number
            .first_max_by_key(|&(_, &num)| num)
            .unwrap()
            .0;
        // Find the highest number at position index_of_highest_number+1 to the end
        let second_highest_index = numbers
            .iter()
            .enumerate()
            .skip(index_of_highest_number + 1)
            .max_by_key(|&(_, &num)| num)
            .unwrap()
            .0;
        let number = numbers[index_of_highest_number] * 10 + numbers[second_highest_index];
        println!(
            "{line} Number: {} {index_of_highest_number} {second_highest_index}",
            number
        );
        sum += number;
    }
    sum
}
fn part_b(question: Question) -> u64 {
    println!("Starting Part B");
    let input_file = get_question_data_lines(ChallengeDay::Day3, question);
    // Each line is a series of numbers, we can turn on exactly two
    let mut sum = 0;
    for line in input_file {
        // Convert each character to an integer
        let numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        // Find best big starter number, since we want to start with the optimal local-maxima
        let (mut last_index, value) = numbers
            .iter()
            .enumerate()
            .take(numbers.len() - 12) // Cant go closer to the end than our 12 limit
            .first_max_by_key(|&(_, &num)| num)
            .unwrap();

        let mut number = *value as u64; // Start building the number
        // Collect and find the next 11 numbers to pack in to the right, finding the next local-maxima
        for nth in 0..11 {
            // Can only scan as close to the end as we have numbers remaining
            let index_limit = numbers.len() - last_index - (11 - nth);

            let (new_index, value) = numbers
                .iter()
                .enumerate()
                .skip(last_index + 1)
                .take(index_limit)
                .first_max_by_key(|&(_, &num)| num)
                .unwrap();
            last_index = new_index;
            number = (number * 10) + (*value as u64);
        }

        println!("{line} Number: {}", number);
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 357);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 3121910778619);
    }
}
