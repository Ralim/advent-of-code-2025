use array2d::Array2D;

use crate::{ChallengeDay, input_files::Question};

pub fn get_question_data_lines(day: ChallengeDay, question: Question) -> Vec<String> {
    let file_path = day.get_question_file_path(question);
    let contents = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}

pub fn get_question_data_line(day: ChallengeDay, question: Question) -> String {
    let file_path = day.get_question_file_path(question);
    std::fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path))
}

/// Opens the file, reads all lines, groups lines on empty lines so that you get multiple sets based on empty line breaks
pub fn get_question_data_lines_split_lb(day: ChallengeDay, question: Question) -> Vec<Vec<String>> {
    let file_path = day.get_question_file_path(question);
    let contents = std::fs::read_to_string(&file_path).expect("Failed to read file: {}");

    let line_strings: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    line_strings
        .split(|line| line.is_empty())
        .filter(|lines| !lines.is_empty())
        .map(|lines| lines.to_owned())
        .collect()
}
pub fn get_question_data_as_2d_matrices_lb_sep(
    day: ChallengeDay,
    question: Question,
) -> Vec<Array2D<u8>> {
    let lines = get_question_data_lines_split_lb(day, question);
    // Break on empty lines
    lines
        .into_iter()
        .map(|lines| {
            // Convert lines from &[&str} to Vec<Vec<u8>>
            let rows: Vec<Vec<u8>> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();
            println!("Rows: {:?}", rows);
            Array2D::from_rows(&rows).unwrap()
        })
        .collect()
}
pub fn get_question_data_to_grid(day: ChallengeDay, question: Question) -> Array2D<u8> {
    let file_path = day.get_question_file_path(question);
    let file_contents = std::fs::read_to_string(&file_path).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let filtered_file = file_contents.replace("\n", "").replace("\r", "");

    Array2D::from_row_major(filtered_file.as_bytes(), lines[0].len(), lines.len()).unwrap()
}

pub fn get_question_data_to_num_grid(day: ChallengeDay, question: Question) -> Array2D<i64> {
    let file_path = day.get_question_file_path(question);
    let file_contents = std::fs::read_to_string(&file_path).unwrap();
    let lines: Vec<&str> = file_contents.lines().collect();
    let filtered_file = file_contents.replace("\n", "").replace("\r", "");
    let values: Vec<i64> = filtered_file
        .as_bytes()
        .iter()
        .map(|&a| (a - b'0') as i64)
        .collect();

    Array2D::from_row_major(&values, lines[0].len(), lines.len()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_files::Question;

    #[test]
    fn test_get_day_input_file_lines() {
        let lines = get_question_data_lines(ChallengeDay::Test, Question::Question);
        assert!(!lines.is_empty());
        assert!(lines.iter().all(|line| line.is_ascii()));
    }
    #[test]
    fn test_get_question_data_as_2d_matrices_lb_sep() {
        let matrices =
            get_question_data_as_2d_matrices_lb_sep(ChallengeDay::Test, Question::ManyMatrix);
        assert!(!matrices.is_empty());
        let expected_matrix = Array2D::from_row_major(
            &[
                35, 35, 35, 35, 35, 46, 49, 49, 49, 49, 46, 49, 49, 49, 49, 46, 49, 49, 49, 49, 46,
                51, 46, 51, 46, 46, 51, 46, 46, 46, 46, 46, 46, 46, 46,
            ],
            7,
            5,
        )
        .unwrap();
        assert_eq!(matrices[0], expected_matrix);
        let expected_matrix = Array2D::from_row_major(
            &[
                35, 35, 35, 35, 35, 50, 46, 50, 46, 46, 46, 50, 46, 50, 46, 46, 46, 46, 50, 46, 46,
                46, 46, 50, 46, 46, 46, 46, 50, 46, 46, 46, 46, 46, 46,
            ],
            7,
            5,
        )
        .unwrap();
        assert_eq!(matrices[1], expected_matrix);
    }

    #[test]
    fn test_get_question_data_lines_split_lb() {
        let line_groups =
            get_question_data_lines_split_lb(ChallengeDay::Test, Question::ManyMatrix);
        assert!(!line_groups.is_empty());
        assert_eq!(line_groups.len(), 2);

        // Check first group
        assert_eq!(line_groups[0].len(), 7);
        assert_eq!(line_groups[0][0], "#####");
        assert_eq!(line_groups[0][1], ".1111");
        assert_eq!(line_groups[0][2], ".1111");
        assert_eq!(line_groups[0][3], ".1111");
        assert_eq!(line_groups[0][4], ".3.3.");

        // Check second group
        assert_eq!(line_groups[1].len(), 7);
        assert_eq!(line_groups[1][0], "#####");
        assert_eq!(line_groups[1][1], "2.2..");
        assert_eq!(line_groups[1][2], ".2.2.");
        assert_eq!(line_groups[1][3], "...2.");
        assert_eq!(line_groups[1][4], "...2.");
    }
}
