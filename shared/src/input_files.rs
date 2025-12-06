#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Question {
    Question,
    Sample,
    #[cfg(test)]
    ManyMatrix,
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Question::Question => "input",
            Question::Sample => "sample",
            #[cfg(test)]
            Question::ManyMatrix => "many_matrix",
        };
        write!(f, "{}", s)
    }
}
