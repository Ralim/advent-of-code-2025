use crate::input_files::Question;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ChallengeDay {
    Day1 = 1,
    Day2 = 2,
    Day3 = 3,
    Day4 = 4,
    Day5 = 5,
    Day6 = 6,
    Day7 = 7,
    Day8 = 8,
    Day9 = 9,
    Day10 = 10,
    Day11 = 11,
    Day12 = 12,
    Test = 0, // For test data that can be checked into the repo
}
impl ChallengeDay {
    pub fn get_question_file_path(&self, q: Question) -> String {
        format!("{MANIFEST_DIR}/../input_data/day{}/{q}.txt", *self as u8)
    }
}
impl std::fmt::Display for ChallengeDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}", *self as u8)
    }
}
