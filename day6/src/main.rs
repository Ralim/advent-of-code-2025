use shared::{ChallengeDay, Question, get_question_data_lines, get_question_data_to_grid};
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

fn part_a(question: Question) -> i64 {
    println!("Starting Part A");
    let input_file = get_question_data_lines(ChallengeDay::Day6, question);
    // Split each line by whitespace
    let mut inputs: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<String> = Vec::new();
    for line in input_file {
        if line.contains('*') || line.contains('*') {
            //Terminating line, all operations are here
            operations = line.split_whitespace().map(|s| s.to_string()).collect();
        } else {
            // Number lines
            for (i, num) in line.split_whitespace().enumerate() {
                if inputs.len() < i + 1 {
                    inputs.push(vec![num.parse().unwrap()]);
                } else {
                    inputs[i].push(num.parse().unwrap());
                }
            }
        }
    }

    // Now we have the set of numbers and their operands
    // Zip these together and perform the operand
    let mut total = 0;
    for (operation, numbers) in operations.iter().zip(inputs.iter()) {
        total += match operation.as_str() {
            "+" => (*numbers).iter().sum(),
            "*" => (*numbers).iter().product(),
            _ => 0,
        };
    }
    total
}

fn part_b(question: Question) -> i64 {
    println!("Starting Part B");
    let input_file = get_question_data_to_grid(ChallengeDay::Day6, question);
    // print_array(&input_file);
    // Walk the array going down columns, from right to left
    // As we walk down a column we back-buffer the current value
    let mut inputs: Vec<Vec<i64>> = Vec::new();
    let mut operations: Vec<u8> = Vec::new();

    let mut current_numbers = Vec::new();
    for column in (0..input_file.num_columns()).rev() {
        let mut current_value = 0;
        for row in 0..input_file.num_rows() {
            let value = input_file.get(row, column).unwrap();
            // Print value as ascii
            if value.is_ascii_digit() {
                // Back-buffer number
                current_value = current_value * 10 + (*value as i64 - '0' as i64);
            } else if *value == b'*' || *value == b'+' {
                //End of the column; and end of the dataset
                operations.push(*value); // Add this op
                current_numbers.push(current_value); // Current value is done, add it to the back-stack
                inputs.push(current_numbers.clone());
                current_numbers.clear();
            } else if row == input_file.num_rows() - 1 && current_value != 0 {
                current_numbers.push(current_value);
            }
        }
    }
    let mut total = 0;
    for (operation, numbers) in operations.iter().zip(inputs.iter()) {
        total += match operation {
            b'+' => (*numbers).iter().sum(),
            b'*' => (*numbers).iter().product(),
            _ => 0,
        };
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a_question1_sample() {
        assert_eq!(part_a(Question::Sample), 4277556);
    }

    #[test]
    fn test_part_b_question2_sample() {
        assert_eq!(part_b(Question::Sample), 3263827);
    }
}
