use crate::blocks::CommandBlock;

pub fn blocks_to_markdown(blocks: &[CommandBlock], pending_line: &str) -> String {
    let mut out = String::new();
    out.push_str("# Terminal Session Export\n\n");

    for block in blocks {
        out.push_str(&format!("## Block #{}\n\n", block.id));
        out.push_str(&format!("- Command: `{}`\n", block.command));
        out.push_str(&format!("- CWD: `{}`\n", block.working_directory));
        out.push_str(&format!("- Timestamp(ms): `{}`\n", block.timestamp_unix_ms));
        out.push_str(&format!(
            "- Exit Code: `{}`\n",
            block
                .exit_code
                .map(|c| c.to_string())
                .unwrap_or_else(|| "None".to_owned())
        ));
        out.push_str(&format!("- Bookmarked: `{}`\n\n", block.bookmarked));

        out.push_str("```text\n");
        for line in &block.output_lines {
            out.push_str(line);
            out.push('\n');
        }
        out.push_str("```\n\n");
    }

    if !pending_line.trim().is_empty() {
        out.push_str("## Pending Line\n\n");
        out.push_str("```text\n");
        out.push_str(pending_line);
        out.push('\n');
        out.push_str("```\n");
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::blocks::CommandBlock;

    use super::blocks_to_markdown;

    #[test]
    fn export_contains_block_content() {
        let mut block = CommandBlock::new(7, "echo hi".to_owned(), "D:\\repo".to_owned());
        block.bookmarked = true;
        block.output_lines.push("hi".to_owned());

        let markdown = blocks_to_markdown(&[block], "pending");
        assert!(markdown.contains("Block #7"));
        assert!(markdown.contains("echo hi"));
        assert!(markdown.contains("Bookmarked: `true`"));
        assert!(markdown.contains("Pending Line"));
    }
}
