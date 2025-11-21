use crate::{ChallengeDay, input_files::Question};

pub fn get_question_data_lines(day: ChallengeDay, question: Question) -> Vec<String> {
    let file_path = day.get_question_file_path(question);
    let contents =
        std::fs::read_to_string(&file_path).expect(&format!("Failed to read file: {}", file_path));
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    return lines;
}

pub fn get_question_data_as_2d_matrix(day: ChallengeDay, question: Question) -> Vec<Vec<u8>> {
    let file_path = day.get_question_file_path(question);
    let contents =
        std::fs::read_to_string(&file_path).expect(&format!("Failed to read file: {}", file_path));
    let bytes: Vec<Vec<u8>> = contents.lines().map(|s| s.bytes().collect()).collect();
    return bytes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_files::Question;

    #[test]
    fn test_get_day_input_file_lines() {
        let lines = get_question_data_lines(ChallengeDay::Test, Question::Question1);
        assert!(!lines.is_empty());
        assert!(lines.iter().all(|line| line.is_ascii()));
    }

    #[test]
    fn test_get_day_input_file_bytes() {
        let bytes = get_question_data_as_2d_matrix(ChallengeDay::Test, Question::Question1);
        assert!(!bytes.is_empty());
        assert!(
            bytes
                .iter()
                .all(|byte_line| !byte_line.is_empty() || bytes.len() == 1)
        );
    }

    #[test]
    fn test_consistency_between_lines_and_bytes() {
        let lines = get_question_data_lines(ChallengeDay::Test, Question::Question1);
        let bytes = get_question_data_as_2d_matrix(ChallengeDay::Test, Question::Question1);

        assert_eq!(lines.len(), bytes.len());

        for (line, byte_line) in lines.iter().zip(bytes.iter()) {
            assert_eq!(line.as_bytes(), byte_line.as_slice());
        }
        assert_eq!(
            bytes,
            vec![
                vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57],
                vec![57, 56, 55, 54, 53, 52, 51, 50, 49, 48]
            ]
        );
    }
}
