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
fn lines_to_deltas(data: &Vec<String>) -> Vec<i32> {
    // For each non-blank line, match first char as L/R, then rest as the integer part.
    // If L invert the number
    data.iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let distance = distance.parse::<i32>().unwrap();
            if direction == "L" {
                -distance
            } else {
                distance
            }
        })
        .collect()
}
fn part_a(question: Question) -> i32 {
    println!("Starting Part A");
    let input_file = get_question_data_lines(ChallengeDay::Day1, question);
    let instructions = lines_to_deltas(&input_file);
    let mut dial = 50;
    let mut zero_counter = 0;
    for instruction in instructions {
        dial += instruction;
        while dial < 0 {
            dial += 100;
        }
        dial %= 100;
        if dial == 0 {
            zero_counter += 1;
        }
    }
    println!("A: Final dial value: {}", dial);
    println!("A: Number of times dial reached zero: {}", zero_counter);
    zero_counter
}
fn part_b(question: Question) -> i32 {
    println!("Starting Part B");
    let input_file = get_question_data_lines(ChallengeDay::Day1, question);
    let instructions = lines_to_deltas(&input_file);
    let mut dial = 50;
    let mut zero_counter = 0;
    for instruction in instructions {
        let whole_loops = (instruction / 100).abs();
        let remainder = instruction % 100; // Grab remaining delta
        zero_counter += whole_loops;
        let start = dial;
        dial += remainder;
        if dial > 99 {
            dial -= 100;
            zero_counter += 1;
        } else if dial < 0 {
            dial += 100;
            if start != 0 {
                zero_counter += 1;
            }
        } else if dial == 0 {
            zero_counter += 1;
        }
    }
    println!("B: Final dial value: {}", dial);
    println!("B: Number of times dial passed zero: {}", zero_counter);
    zero_counter
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
        assert_eq!(part_b(Question::Sample), 6);
    }
}
