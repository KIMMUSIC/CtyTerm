use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum AiTool {
    ClaudeCode,
    CodexCli,
}

impl AiTool {
    pub fn label(self) -> &'static str {
        match self {
            AiTool::ClaudeCode => "Claude Code",
            AiTool::CodexCli => "Codex CLI",
        }
    }

    pub fn binary_name(self) -> &'static str {
        match self {
            AiTool::ClaudeCode => "claude",
            AiTool::CodexCli => "codex",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum AiBlockStatus {
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiBlock {
    pub id: u64,
    pub tool: AiTool,
    pub prompt: String,
    pub context_block_ids: Vec<u64>,
    pub output_lines: Vec<String>,
    pub status: AiBlockStatus,
    pub exit_code: Option<i32>,
    pub started_unix_ms: u64,
    pub duration_ms: Option<u64>,
}

impl AiBlock {
    pub fn new(id: u64, tool: AiTool, prompt: String, context_block_ids: Vec<u64>) -> Self {
        Self {
            id,
            tool,
            prompt,
            context_block_ids,
            output_lines: Vec::new(),
            status: AiBlockStatus::Running,
            exit_code: None,
            started_unix_ms: unix_ms_now(),
            duration_ms: None,
        }
    }

    pub fn append_output_lines(&mut self, lines: &[String]) {
        self.output_lines.extend(lines.iter().cloned());
    }

    pub fn complete(&mut self, exit_code: i32, duration_ms: u64) {
        self.status = AiBlockStatus::Completed;
        self.exit_code = Some(exit_code);
        self.duration_ms = Some(duration_ms);
    }

    pub fn fail(&mut self, message: String, duration_ms: u64) {
        self.status = AiBlockStatus::Failed;
        self.exit_code = Some(-1);
        self.duration_ms = Some(duration_ms);
        self.output_lines.push(message);
    }
}

fn unix_ms_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis().min(u64::MAX as u128) as u64)
        .unwrap_or_default()
}
