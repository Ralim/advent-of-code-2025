use rayon::prelude::*;
use shared::{ChallengeDay, Question, get_question_data_line};

fn parse_pair(pair: &str) -> (u64, u64) {
    let (start, stop) = pair.trim().split_once('-').unwrap();
    let start = start.parse::<u64>().unwrap();
    let stop = stop.parse::<u64>().unwrap();
    (start, stop)
}

fn get_pairs(question: Question) -> Vec<(u64, u64)> {
    let input_file = get_question_data_line(ChallengeDay::Day2, question);

    input_file
        .split(",")
        .filter(|line| !line.is_empty())
        .map(parse_pair)
        .collect::<Vec<(u64, u64)>>()
}
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

fn part_a(question: Question) -> u64 {
    println!("Starting Part A");
    let data = get_pairs(question);

    let mut invalid_id_count = 0;
    for pair in data {
        let (start, stop) = pair;
        for x in start..=stop {
            if (x.ilog10() + 1) % 2 == 0 {
                // Even number of digits
                // Check if the stringification of it a pattern repeating twice like 7878
                let s = x.to_string();
                let first = s.chars().take(s.len() / 2);
                let second = s.chars().skip(s.len() / 2);
                if first.eq(second) {
                    invalid_id_count += x;
                }
            }
        }
    }
    invalid_id_count
}
fn part_b(question: Question) -> u64 {
    println!("Starting Part B");
    let data = get_pairs(question);
    let mut invalid_id_count = 0;

    for pair in data {
        let (start, stop) = pair;
        for x in start..=stop {
            // Even number of digits
            // Check if the stringification of it a pattern repeating twice like 7878 or 787878
            let s = x.to_string();
            for segment_length in 1..s.len() {
                let tail_len = s.len() - segment_length;
                if (tail_len) % segment_length != 0 {
                    continue;
                }
                let first = s.chars().take(segment_length);
                let second = s.chars().skip(segment_length);

                // Check first repeated repeats times is second
                let extended_first = first.cycle().take(tail_len);
                if extended_first.eq(second) {
                    invalid_id_count += x;
                    break;
                }
            }
        }
    }

    invalid_id_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 1227775554);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 4174379265);
    }
}
