use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandHistory {
    commands: Vec<String>,
}

impl CommandHistory {
    pub fn push(&mut self, command: String) {
        if !command.trim().is_empty() {
            self.commands.push(command);
        }
    }

    pub fn recent(&self, max_items: usize) -> Vec<String> {
        let start = self.commands.len().saturating_sub(max_items);
        self.commands[start..].to_vec()
    }

    pub fn search(&self, query: &str, max_items: usize) -> Vec<String> {
        let normalized = query.trim().to_lowercase();
        let mut out = Vec::new();

        for cmd in self.commands.iter().rev() {
            let matches = if normalized.is_empty() {
                true
            } else {
                cmd.to_lowercase().contains(&normalized)
            };

            if matches && !out.iter().any(|existing| existing == cmd) {
                out.push(cmd.clone());
            }

            if out.len() >= max_items {
                break;
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::CommandHistory;

    #[test]
    fn search_returns_most_recent_unique_matches() {
        let mut history = CommandHistory::default();
        history.push("cargo check".to_owned());
        history.push("git status".to_owned());
        history.push("cargo test".to_owned());
        history.push("cargo check".to_owned());

        let matches = history.search("cargo", 10);
        assert_eq!(matches, vec!["cargo check", "cargo test"]);
    }

    #[test]
    fn empty_query_returns_recent_entries() {
        let mut history = CommandHistory::default();
        history.push("one".to_owned());
        history.push("two".to_owned());
        history.push("three".to_owned());

        let matches = history.search("", 2);
        assert_eq!(matches, vec!["three", "two"]);
    }
}
