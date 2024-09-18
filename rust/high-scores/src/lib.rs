#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted: Vec<u32> = self.scores.clone();
        sorted.sort();
        sorted.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted: Vec<u32> = self.scores.clone();
        sorted.sort();
        sorted.iter().rev().cloned().take(3).collect()
    }
}
