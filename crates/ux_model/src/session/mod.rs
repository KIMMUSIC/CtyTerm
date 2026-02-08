use crate::ai::{AiBlock, AiTool};
use crate::blocks::CommandBlock;
use crate::export::blocks_to_markdown;
use crate::history::CommandHistory;
use crate::search::search_blocks;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SessionState {
    blocks: Vec<CommandBlock>,
    ai_blocks: Vec<AiBlock>,
    history: CommandHistory,
    next_block_id: u64,
    next_ai_block_id: u64,
    pending_line: String,
}

#[derive(Debug, Clone)]
pub enum TimelineItem {
    Command(CommandBlock),
    Ai(AiBlock),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSnapshot {
    pub blocks: Vec<CommandBlock>,
    pub ai_blocks: Vec<AiBlock>,
    pub history: CommandHistory,
    pub next_block_id: u64,
    pub next_ai_block_id: u64,
    pub pending_line: String,
}

impl SessionState {
    pub fn new(_initial_cwd: String) -> Self {
        Self {
            blocks: Vec::new(),
            ai_blocks: Vec::new(),
            history: CommandHistory::default(),
            next_block_id: 0,
            next_ai_block_id: 1,
            pending_line: String::new(),
        }
    }

    pub fn start_command_block(&mut self, command: String, cwd: String) {
        self.history.push(command.clone());
        self.blocks
            .push(CommandBlock::new(self.next_block_id, command, cwd));
        self.next_block_id += 1;
    }

    pub fn push_output_lines(&mut self, lines: Vec<String>) {
        if lines.is_empty() {
            return;
        }

        if let Some(last_block) = self.blocks.last_mut() {
            last_block.append_output(&lines);
        }
    }

    pub fn set_pending_line(&mut self, line: String) {
        self.pending_line = line;
    }

    pub fn visible_lines(&self, max_lines: usize) -> Vec<String> {
        let mut all_lines = Vec::new();

        for block in &self.blocks {
            if !block.command.is_empty() {
                all_lines.push(format!("$ {}", block.command));
            }
            all_lines.extend(block.output_lines.iter().cloned());
        }

        if !self.pending_line.is_empty() {
            all_lines.push(self.pending_line.clone());
        }

        let start = all_lines.len().saturating_sub(max_lines);
        all_lines[start..].to_vec()
    }

    pub fn history_recent(&self, max_items: usize) -> Vec<String> {
        self.history.recent(max_items)
    }

    pub fn history_search(&self, query: &str, max_items: usize) -> Vec<String> {
        self.history.search(query, max_items)
    }

    pub fn blocks(&self) -> &[CommandBlock] {
        &self.blocks
    }

    pub fn block_by_id(&self, block_id: u64) -> Option<&CommandBlock> {
        self.blocks.iter().find(|block| block.id == block_id)
    }

    pub fn ai_blocks(&self) -> &[AiBlock] {
        &self.ai_blocks
    }

    pub fn ai_block_by_id(&self, block_id: u64) -> Option<&AiBlock> {
        self.ai_blocks.iter().find(|block| block.id == block_id)
    }

    pub fn pending_line(&self) -> &str {
        &self.pending_line
    }

    pub fn search_block_ids(&self, query: &str, max_items: usize) -> Vec<u64> {
        let mut ids = search_blocks(&self.blocks, query);
        ids.reverse();
        if ids.len() > max_items {
            ids.truncate(max_items);
        }
        ids
    }

    pub fn toggle_bookmark(&mut self, block_id: u64) -> Option<bool> {
        let block = self.blocks.iter_mut().find(|b| b.id == block_id)?;
        block.bookmarked = !block.bookmarked;
        Some(block.bookmarked)
    }

    pub fn remove_command_block(&mut self, block_id: u64) -> bool {
        let original_len = self.blocks.len();
        self.blocks.retain(|block| block.id != block_id);
        self.blocks.len() != original_len
    }

    pub fn remove_ai_block(&mut self, ai_block_id: u64) -> bool {
        let original_len = self.ai_blocks.len();
        self.ai_blocks.retain(|block| block.id != ai_block_id);
        self.ai_blocks.len() != original_len
    }

    pub fn clear_timeline(&mut self) {
        self.blocks.clear();
        self.ai_blocks.clear();
        self.pending_line.clear();
    }

    pub fn bookmarked_count(&self) -> usize {
        self.blocks.iter().filter(|b| b.bookmarked).count()
    }

    pub fn start_ai_block(
        &mut self,
        tool: AiTool,
        prompt: String,
        context_block_ids: Vec<u64>,
    ) -> u64 {
        let ai_id = self.next_ai_block_id;
        self.ai_blocks
            .push(AiBlock::new(ai_id, tool, prompt, context_block_ids));
        self.next_ai_block_id += 1;
        ai_id
    }

    pub fn append_ai_output_lines(&mut self, ai_block_id: u64, lines: &[String]) -> bool {
        let Some(block) = self
            .ai_blocks
            .iter_mut()
            .find(|block| block.id == ai_block_id)
        else {
            return false;
        };

        block.append_output_lines(lines);
        true
    }

    pub fn complete_ai_block(
        &mut self,
        ai_block_id: u64,
        exit_code: i32,
        duration_ms: u64,
    ) -> bool {
        let Some(block) = self
            .ai_blocks
            .iter_mut()
            .find(|block| block.id == ai_block_id)
        else {
            return false;
        };

        block.complete(exit_code, duration_ms);
        true
    }

    pub fn fail_ai_block(&mut self, ai_block_id: u64, message: String, duration_ms: u64) -> bool {
        let Some(block) = self
            .ai_blocks
            .iter_mut()
            .find(|block| block.id == ai_block_id)
        else {
            return false;
        };

        block.fail(message, duration_ms);
        true
    }

    pub fn ai_block_count(&self) -> usize {
        self.ai_blocks.len()
    }

    pub fn build_context_payload(&self, block_ids: &[u64], max_lines_per_block: usize) -> String {
        if block_ids.is_empty() {
            return String::new();
        }

        let mut out = String::new();
        out.push_str("Attached terminal context blocks:\n");
        out.push_str("================================\n");

        for block_id in block_ids {
            if let Some(block) = self.block_by_id(*block_id) {
                out.push_str(&format!("Block #{}\n", block.id));
                out.push_str(&format!("Command: {}\n", block.command));
                out.push_str(&format!("CWD: {}\n", block.working_directory));
                out.push_str("Output:\n");

                let start = block.output_lines.len().saturating_sub(max_lines_per_block);
                for line in &block.output_lines[start..] {
                    out.push_str("  ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push('\n');
            }
        }

        out
    }

    pub fn timeline_items(&self) -> Vec<TimelineItem> {
        let mut items = Vec::with_capacity(self.blocks.len() + self.ai_blocks.len());
        for block in &self.blocks {
            items.push(TimelineItem::Command(block.clone()));
        }
        for ai in &self.ai_blocks {
            items.push(TimelineItem::Ai(ai.clone()));
        }

        items.sort_by_key(|item| match item {
            TimelineItem::Command(block) => (block.timestamp_unix_ms, 0_u8, block.id),
            TimelineItem::Ai(block) => (block.started_unix_ms, 1_u8, block.id),
        });

        items
    }

    pub fn export_markdown(&self, bookmarks_only: bool) -> String {
        if bookmarks_only {
            let blocks: Vec<CommandBlock> = self
                .blocks
                .iter()
                .filter(|block| block.bookmarked)
                .cloned()
                .collect();
            return blocks_to_markdown(&blocks, self.pending_line());
        }

        blocks_to_markdown(&self.blocks, self.pending_line())
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn to_snapshot(&self) -> SessionSnapshot {
        SessionSnapshot {
            blocks: self.blocks.clone(),
            ai_blocks: self.ai_blocks.clone(),
            history: self.history.clone(),
            next_block_id: self.next_block_id,
            next_ai_block_id: self.next_ai_block_id,
            pending_line: self.pending_line.clone(),
        }
    }

    pub fn from_snapshot(snapshot: SessionSnapshot) -> Self {
        let mut blocks = snapshot.blocks;
        if blocks.first().is_some_and(|block| {
            block.id == 0
                && block.command == "<shell-session>"
                && block.output_lines.is_empty()
                && !block.bookmarked
        }) {
            blocks.remove(0);
        }

        Self {
            blocks,
            ai_blocks: snapshot.ai_blocks,
            history: snapshot.history,
            next_block_id: snapshot.next_block_id,
            next_ai_block_id: snapshot.next_ai_block_id,
            pending_line: snapshot.pending_line,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::{AiBlockStatus, AiTool};
    use crate::blocks::CommandBlock;

    use super::SessionState;

    #[test]
    fn bookmark_toggle_changes_state() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo 1".to_owned(), "D:\\repo".to_owned());

        let changed = session.toggle_bookmark(0);
        assert_eq!(changed, Some(true));
        assert_eq!(session.bookmarked_count(), 1);

        let changed = session.toggle_bookmark(0);
        assert_eq!(changed, Some(false));
        assert_eq!(session.bookmarked_count(), 0);
    }

    #[test]
    fn block_search_returns_recent_matches_first() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("cargo check".to_owned(), "D:\\repo".to_owned());
        session.push_output_lines(vec!["ok".to_owned()]);
        session.start_command_block("cargo test".to_owned(), "D:\\repo".to_owned());
        session.push_output_lines(vec!["pass".to_owned()]);

        let ids = session.search_block_ids("cargo", 10);
        assert_eq!(ids, vec![1, 0]);
    }

    #[test]
    fn export_bookmarks_only_excludes_unbookmarked_blocks() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo a".to_owned(), "D:\\repo".to_owned());
        session.push_output_lines(vec!["a".to_owned()]);
        session.start_command_block("echo b".to_owned(), "D:\\repo".to_owned());
        session.push_output_lines(vec!["b".to_owned()]);
        session.toggle_bookmark(1);

        let markdown = session.export_markdown(true);
        assert!(markdown.contains("echo b"));
        assert!(!markdown.contains("echo a"));
    }

    #[test]
    fn ai_block_lifecycle_works() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        let ai_id = session.start_ai_block(AiTool::CodexCli, "summarize".to_owned(), vec![0]);
        let appended = session.append_ai_output_lines(ai_id, &["answer".to_owned()]);
        assert!(appended);

        let completed = session.complete_ai_block(ai_id, 0, 120);
        assert!(completed);

        let ai = session.ai_block_by_id(ai_id).expect("ai block must exist");
        assert_eq!(ai.status, AiBlockStatus::Completed);
        assert_eq!(ai.exit_code, Some(0));
        assert_eq!(session.ai_block_count(), 1);
    }

    #[test]
    fn context_payload_contains_selected_blocks() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo alpha".to_owned(), "D:\\repo".to_owned());
        session.push_output_lines(vec!["alpha".to_owned()]);

        let payload = session.build_context_payload(&[0], 20);
        assert!(payload.contains("Block #0"));
        assert!(payload.contains("echo alpha"));
        assert!(payload.contains("alpha"));
    }

    #[test]
    fn snapshot_roundtrip_preserves_state() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo a".to_owned(), "D:\\repo".to_owned());
        session.push_output_lines(vec!["a".to_owned()]);
        session.toggle_bookmark(0);
        let ai_id = session.start_ai_block(AiTool::CodexCli, "sum".to_owned(), vec![0]);
        session.append_ai_output_lines(ai_id, &["ok".to_owned()]);
        session.complete_ai_block(ai_id, 0, 10);

        let snapshot = session.to_snapshot();
        let restored = SessionState::from_snapshot(snapshot);

        assert_eq!(restored.block_count(), session.block_count());
        assert_eq!(restored.ai_block_count(), session.ai_block_count());
        assert_eq!(restored.bookmarked_count(), session.bookmarked_count());
        assert_eq!(restored.pending_line(), session.pending_line());
    }

    #[test]
    fn snapshot_restore_drops_legacy_shell_session_placeholder() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo hello".to_owned(), "D:\\repo".to_owned());
        let mut snapshot = session.to_snapshot();

        snapshot.blocks.insert(
            0,
            CommandBlock::new(0, "<shell-session>".to_owned(), "D:\\repo".to_owned()),
        );
        snapshot.next_block_id = 1;

        let restored = SessionState::from_snapshot(snapshot);
        assert_eq!(restored.block_count(), 1);
        assert_eq!(restored.blocks()[0].command, "echo hello");
        assert_eq!(restored.blocks()[0].id, 0);
    }

    #[test]
    fn remove_command_block_deletes_matching_block_only() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo a".to_owned(), "D:\\repo".to_owned());
        session.start_command_block("echo b".to_owned(), "D:\\repo".to_owned());

        assert!(session.remove_command_block(0));
        assert!(!session.remove_command_block(99));
        assert_eq!(session.block_count(), 1);
        assert_eq!(session.blocks()[0].id, 1);
        assert_eq!(session.blocks()[0].command, "echo b");
    }

    #[test]
    fn clear_timeline_removes_command_ai_and_pending_line() {
        let mut session = SessionState::new("D:\\repo".to_owned());
        session.start_command_block("echo x".to_owned(), "D:\\repo".to_owned());
        let ai_id = session.start_ai_block(AiTool::CodexCli, "sum".to_owned(), vec![0]);
        session.append_ai_output_lines(ai_id, &["ok".to_owned()]);
        session.set_pending_line("pending".to_owned());

        session.clear_timeline();

        assert_eq!(session.block_count(), 0);
        assert_eq!(session.ai_block_count(), 0);
        assert!(session.pending_line().is_empty());
    }
}
