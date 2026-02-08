#[derive(Debug, Clone)]
pub struct TextGrid {
    width: usize,
    height: usize,
    lines: Vec<String>,
}

impl TextGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            lines: Vec::new(),
        }
    }

    pub fn set_lines(&mut self, lines: &[String]) {
        let start = lines.len().saturating_sub(self.height);
        self.lines = lines[start..]
            .iter()
            .map(|line| line.chars().take(self.width).collect())
            .collect();
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}

impl Default for TextGrid {
    fn default() -> Self {
        Self::new(140, 120)
    }
}
