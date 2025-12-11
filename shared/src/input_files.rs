#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Question {
    Question,
    Sample,
    AltSample,
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Question::Question => "input",
            Question::Sample => "sample",
            Question::AltSample => "alt_sample",
        };
        write!(f, "{}", s)
    }
}
