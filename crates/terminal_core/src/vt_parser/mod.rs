#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParseState {
    Ground,
    Escape,
    Csi,
    Osc,
    OscEscape,
}

pub struct MinimalVtParser {
    state: ParseState,
    current_line: String,
    saw_cr: bool,
}

impl Default for MinimalVtParser {
    fn default() -> Self {
        Self {
            state: ParseState::Ground,
            current_line: String::new(),
            saw_cr: false,
        }
    }
}

impl MinimalVtParser {
    pub fn feed(&mut self, chunk: &[u8]) -> Vec<String> {
        let mut completed = Vec::new();

        for &byte in chunk {
            match self.state {
                ParseState::Ground => match byte {
                    0x1B => self.state = ParseState::Escape,
                    b'\r' => {
                        self.saw_cr = true;
                    }
                    b'\n' => {
                        self.saw_cr = false;
                        completed.push(std::mem::take(&mut self.current_line));
                    }
                    0x08 => {
                        self.saw_cr = false;
                        self.current_line.pop();
                    }
                    b'\t' => {
                        if self.saw_cr {
                            self.current_line.clear();
                            self.saw_cr = false;
                        }
                        self.saw_cr = false;
                        self.current_line.push_str("    ");
                    }
                    b if b.is_ascii() && !b.is_ascii_control() => {
                        if self.saw_cr {
                            // CR without LF means cursor returned to column 0.
                            // Approximate this by replacing the current line content.
                            self.current_line.clear();
                        }
                        self.saw_cr = false;
                        self.current_line.push(b as char);
                    }
                    _ => {}
                },
                ParseState::Escape => match byte {
                    b'[' => self.state = ParseState::Csi,
                    b']' => self.state = ParseState::Osc,
                    _ => self.state = ParseState::Ground,
                },
                ParseState::Csi => {
                    if (0x40..=0x7E).contains(&byte) {
                        self.state = ParseState::Ground;
                    }
                }
                ParseState::Osc => match byte {
                    0x07 => self.state = ParseState::Ground,
                    0x1B => self.state = ParseState::OscEscape,
                    _ => {}
                },
                ParseState::OscEscape => match byte {
                    b'\\' => self.state = ParseState::Ground,
                    _ => self.state = ParseState::Osc,
                },
            }
        }

        completed
    }

    pub fn current_line(&self) -> &str {
        &self.current_line
    }
}

#[cfg(test)]
mod tests {
    use super::MinimalVtParser;

    #[test]
    fn strips_ansi_sequences() {
        let mut parser = MinimalVtParser::default();
        let lines = parser.feed(b"\x1b[31mhello\x1b[0m\n");
        assert_eq!(lines, vec!["hello"]);
    }

    #[test]
    fn handles_crlf_without_dropping_line_content() {
        let mut parser = MinimalVtParser::default();
        let lines = parser.feed(b"alpha\r\nbeta\r\n");
        assert_eq!(lines, vec!["alpha", "beta"]);
    }

    #[test]
    fn carriage_return_rewrites_the_same_line() {
        let mut parser = MinimalVtParser::default();
        let lines = parser.feed(b"first\rsecond\n");
        assert_eq!(lines, vec!["second"]);
    }
}
