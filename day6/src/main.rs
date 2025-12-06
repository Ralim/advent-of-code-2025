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
    let _input_file = get_question_data_lines(ChallengeDay::Day6, question);
    0
}

fn part_b(question: Question) -> usize {
    println!("Starting Part B");
    let _input_file = get_question_data_lines(ChallengeDay::Day6, question);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 0);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 0);
    }
}
