use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ScrollbackBuffer {
    capacity: usize,
    lines: VecDeque<String>,
}

impl ScrollbackBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            lines: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push_line(&mut self, line: String) {
        if self.lines.len() >= self.capacity {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    pub fn extend(&mut self, lines: impl IntoIterator<Item = String>) {
        for line in lines {
            self.push_line(line);
        }
    }

    pub fn tail(&self, max_lines: usize) -> Vec<String> {
        let start = self.lines.len().saturating_sub(max_lines);
        self.lines.iter().skip(start).cloned().collect()
    }
}

impl Default for ScrollbackBuffer {
    fn default() -> Self {
        Self::new(20_000)
    }
}
