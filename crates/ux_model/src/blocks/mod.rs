use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBlock {
    pub id: u64,
    pub command: String,
    pub output_lines: Vec<String>,
    pub bookmarked: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: Option<u64>,
    pub working_directory: String,
    pub timestamp_unix_ms: u64,
}

impl CommandBlock {
    pub fn new(id: u64, command: String, working_directory: String) -> Self {
        Self {
            id,
            command,
            output_lines: Vec::new(),
            bookmarked: false,
            exit_code: None,
            duration_ms: None,
            working_directory,
            timestamp_unix_ms: unix_ms_now(),
        }
    }

    pub fn append_output(&mut self, lines: &[String]) {
        self.output_lines.extend(lines.iter().cloned());
    }
}

fn unix_ms_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().min(u64::MAX as u128) as u64)
        .unwrap_or_default()
}
