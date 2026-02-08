use crate::blocks::CommandBlock;

pub fn search_blocks(blocks: &[CommandBlock], query: &str) -> Vec<u64> {
    if query.trim().is_empty() {
        return Vec::new();
    }

    let query = query.to_lowercase();
    blocks
        .iter()
        .filter(|block| {
            block.command.to_lowercase().contains(&query)
                || block
                    .output_lines
                    .iter()
                    .any(|line| line.to_lowercase().contains(&query))
        })
        .map(|block| block.id)
        .collect()
}
