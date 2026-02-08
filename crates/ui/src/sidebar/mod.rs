use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use eframe::egui;

use crate::theme;
mod devicons;

#[derive(Debug, Clone)]
pub enum SidebarAction {
    OpenFile(PathBuf),
}

#[derive(Debug, Clone)]
struct TreeNode {
    path: PathBuf,
    is_dir: bool,
    children: Vec<TreeNode>,
}

#[derive(Debug, Clone, Copy)]
struct ScanBudget {
    max_depth: usize,
    max_files: usize,
    max_tree_nodes: usize,
    tree_nodes_seen: usize,
}

#[derive(Debug, Clone)]
pub struct SidebarState {
    root_path: PathBuf,
    file_search: String,
    tree_search: String,
    entries: Vec<PathBuf>,
    tree_roots: Vec<TreeNode>,
    git_summary: String,
    scan_error: Option<String>,
    scanned: bool,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            root_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            file_search: String::new(),
            tree_search: String::new(),
            entries: Vec::new(),
            tree_roots: Vec::new(),
            git_summary: "not a repository".to_owned(),
            scan_error: None,
            scanned: false,
        }
    }
}

impl SidebarState {
    fn ensure_scanned(&mut self) {
        if !self.scanned {
            self.refresh();
        }
    }

    fn refresh(&mut self) {
        self.entries.clear();
        self.tree_roots.clear();
        self.scan_error = None;

        let mut budget = ScanBudget {
            max_depth: 5,
            max_files: 1_000,
            max_tree_nodes: 2_000,
            tree_nodes_seen: 0,
        };

        match Self::scan_directory(&self.root_path, 0, &mut budget, &mut self.entries) {
            Ok(nodes) => self.tree_roots = nodes,
            Err(err) => {
                self.scan_error = Some(format!("root scan failed: {err}"));
                self.scanned = true;
                return;
            }
        }

        self.entries.sort();
        self.refresh_git_summary();
        self.scanned = true;
    }

    fn scan_directory(
        dir: &Path,
        depth: usize,
        budget: &mut ScanBudget,
        entries: &mut Vec<PathBuf>,
    ) -> std::io::Result<Vec<TreeNode>> {
        if depth > budget.max_depth || budget.tree_nodes_seen >= budget.max_tree_nodes {
            return Ok(Vec::new());
        }

        let children = Self::read_dir_sorted(dir)?;
        let mut nodes = Vec::new();

        for path in children {
            if budget.tree_nodes_seen >= budget.max_tree_nodes {
                break;
            }

            let is_dir = path.is_dir();
            budget.tree_nodes_seen += 1;

            let mut node = TreeNode {
                path: path.clone(),
                is_dir,
                children: Vec::new(),
            };

            if is_dir {
                if depth < budget.max_depth {
                    if let Ok(child_nodes) = Self::scan_directory(&path, depth + 1, budget, entries)
                    {
                        node.children = child_nodes;
                    }
                }
            } else if path.is_file() && entries.len() < budget.max_files {
                entries.push(path.clone());
            }

            nodes.push(node);
        }

        Ok(nodes)
    }

    fn read_dir_sorted(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        for entry in fs::read_dir(dir)?.flatten() {
            paths.push(entry.path());
        }
        paths.sort_by(|a, b| {
            let a_dir = a.is_dir();
            let b_dir = b.is_dir();
            match (a_dir, b_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });
        Ok(paths)
    }

    fn refresh_git_summary(&mut self) {
        match Command::new("git")
            .arg("-C")
            .arg(&self.root_path)
            .arg("rev-parse")
            .arg("--is-inside-work-tree")
            .output()
        {
            Ok(output) if output.status.success() => {
                let inside = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                if inside != "true" {
                    self.git_summary = "not a repository".to_owned();
                    return;
                }
            }
            Ok(_) => {
                self.git_summary = "not a repository".to_owned();
                return;
            }
            Err(err) => {
                self.git_summary = format!("git unavailable: {err}");
                return;
            }
        }

        let branch_name = match resolve_git_branch_name(&self.root_path) {
            Ok(name) => name,
            Err(message) => {
                self.git_summary = message;
                return;
            }
        };

        let porcelain = Command::new("git")
            .arg("-C")
            .arg(&self.root_path)
            .arg("status")
            .arg("--porcelain")
            .output();

        let output = match porcelain {
            Ok(output) if output.status.success() => output,
            Ok(output) => {
                self.git_summary = format!(
                    "git status error: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                );
                return;
            }
            Err(err) => {
                self.git_summary = format!("git status unavailable: {err}");
                return;
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let (staged, unstaged, untracked) = count_porcelain_changes(&stdout);

        self.git_summary = format!(
            "branch: {branch_name} | staged: {staged} | unstaged: {unstaged} | untracked: {untracked}"
        );
    }

    fn filtered_entries(&self, max_items: usize) -> Vec<&PathBuf> {
        let query = self.file_search.trim().to_lowercase();

        if query.is_empty() {
            return self.entries.iter().take(max_items).collect();
        }

        self.entries
            .iter()
            .filter(|path| path.to_string_lossy().to_lowercase().contains(&query))
            .take(max_items)
            .collect()
    }

    fn rel_display_path<'a>(&'a self, path: &'a Path) -> String {
        path.strip_prefix(&self.root_path)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path.display().to_string())
    }

    fn tree_name(&self, path: &Path) -> String {
        path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| self.rel_display_path(path))
    }

    fn path_matches_tree_query(&self, path: &Path, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }

        self.rel_display_path(path).to_lowercase().contains(query)
    }

    fn tree_node_visible_for_query(&self, node: &TreeNode, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }

        if self.path_matches_tree_query(&node.path, query) {
            return true;
        }

        node.children
            .iter()
            .any(|child| self.tree_node_visible_for_query(child, query))
    }

    fn render_tree_collapsible(
        &self,
        ui: &mut egui::Ui,
        node: &TreeNode,
        actions: &mut Vec<SidebarAction>,
    ) {
        let name = self.tree_name(&node.path);
        if node.is_dir {
            let label = format!("{} {name}", devicons::folder_icon());
            egui::CollapsingHeader::new(
                egui::RichText::new(label)
                    .monospace()
                    .color(theme::ACCENT_BLUE),
            )
            .id_salt(node.path.to_string_lossy().to_string())
            .show(ui, |ui| {
                for child in &node.children {
                    self.render_tree_collapsible(ui, child, actions);
                }
            });
            return;
        }

        let icon = devicons::file_icon(&node.path);
        if ui
            .selectable_label(
                false,
                egui::RichText::new(format!("{icon} {name}"))
                    .monospace()
                    .color(theme::TEXT_PRIMARY),
            )
            .clicked()
        {
            actions.push(SidebarAction::OpenFile(node.path.clone()));
        }
    }

    fn render_tree_filtered(
        &self,
        ui: &mut egui::Ui,
        node: &TreeNode,
        query: &str,
        depth: usize,
        actions: &mut Vec<SidebarAction>,
    ) {
        if !self.tree_node_visible_for_query(node, query) {
            return;
        }

        let icon = if node.is_dir {
            devicons::folder_icon()
        } else {
            devicons::file_icon(&node.path)
        };
        let name = self.tree_name(&node.path);
        let color = if node.is_dir {
            theme::ACCENT_BLUE
        } else {
            theme::TEXT_PRIMARY
        };

        ui.horizontal(|ui| {
            ui.add_space((depth as f32) * 14.0);
            if node.is_dir {
                ui.label(
                    egui::RichText::new(format!("{icon} {name}"))
                        .monospace()
                        .color(color),
                );
            } else if ui
                .selectable_label(
                    false,
                    egui::RichText::new(format!("{icon} {name}"))
                        .monospace()
                        .color(color),
                )
                .clicked()
            {
                actions.push(SidebarAction::OpenFile(node.path.clone()));
            }
        });

        for child in &node.children {
            self.render_tree_filtered(ui, child, query, depth + 1, actions);
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Vec<SidebarAction> {
        self.ensure_scanned();
        let mut actions = Vec::new();

        theme::panel_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("EXPLORER")
                        .monospace()
                        .strong()
                        .color(theme::TEXT_MUTED),
                );
                ui.separator();
                if ui
                    .small_button(egui::RichText::new("refresh").color(theme::ACCENT_BLUE))
                    .clicked()
                {
                    self.scanned = false;
                    self.ensure_scanned();
                }
            });
            ui.separator();

            ui.label(
                egui::RichText::new(format!("workspace: {}", self.root_path.display()))
                    .monospace()
                    .color(theme::TEXT_PRIMARY),
            );
            ui.label(
                egui::RichText::new(format!("git: {}", self.git_summary))
                    .monospace()
                    .color(theme::TEXT_MUTED),
            );

            if let Some(err) = &self.scan_error {
                ui.colored_label(theme::ERROR, err);
            }

            ui.collapsing(
                egui::RichText::new("folder tree")
                    .monospace()
                    .color(theme::TEXT_MUTED),
                |ui| {
                    ui.add_sized(
                        [ui.available_width(), 24.0],
                        egui::TextEdit::singleline(&mut self.tree_search).hint_text("search tree"),
                    );
                    ui.separator();

                    let tree_query = self.tree_search.trim().to_lowercase();

                    egui::ScrollArea::vertical()
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if self.tree_roots.is_empty() {
                                ui.label(
                                    egui::RichText::new("no files")
                                        .monospace()
                                        .color(theme::TEXT_MUTED),
                                );
                                return;
                            }

                            if tree_query.is_empty() {
                                for node in &self.tree_roots {
                                    self.render_tree_collapsible(ui, node, &mut actions);
                                }
                                return;
                            }

                            for node in &self.tree_roots {
                                self.render_tree_filtered(ui, node, &tree_query, 0, &mut actions);
                            }
                        });
                },
            );

            ui.collapsing(
                egui::RichText::new("recent files")
                    .monospace()
                    .color(theme::TEXT_MUTED),
                |ui| {
                    for path in self.entries.iter().take(12) {
                        let icon = devicons::file_icon(path);
                        if ui
                            .selectable_label(
                                false,
                                egui::RichText::new(format!(
                                    "{icon} {}",
                                    self.rel_display_path(path)
                                ))
                                .monospace()
                                .color(theme::TEXT_PRIMARY),
                            )
                            .clicked()
                        {
                            actions.push(SidebarAction::OpenFile(path.clone()));
                        }
                    }
                },
            );

            ui.collapsing(
                egui::RichText::new("file search")
                    .monospace()
                    .color(theme::TEXT_MUTED),
                |ui| {
                    ui.add_sized(
                        [ui.available_width(), 24.0],
                        egui::TextEdit::singleline(&mut self.file_search).hint_text("search files"),
                    );
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .max_height(180.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for path in self.filtered_entries(60) {
                                let icon = devicons::file_icon(path);
                                if ui
                                    .selectable_label(
                                        false,
                                        egui::RichText::new(format!(
                                            "{icon} {}",
                                            self.rel_display_path(path)
                                        ))
                                        .monospace()
                                        .color(theme::TEXT_PRIMARY),
                                    )
                                    .clicked()
                                {
                                    actions.push(SidebarAction::OpenFile(path.clone()));
                                }
                            }
                        });
                },
            );
        });

        actions
    }
}

fn resolve_git_branch_name(root_path: &Path) -> Result<String, String> {
    let branch = Command::new("git")
        .arg("-C")
        .arg(root_path)
        .arg("branch")
        .arg("--show-current")
        .output()
        .map_err(|err| format!("git unavailable: {err}"))?;

    if !branch.status.success() {
        return Err(format!(
            "git error: {}",
            String::from_utf8_lossy(&branch.stderr).trim()
        ));
    }

    let branch_name = String::from_utf8_lossy(&branch.stdout).trim().to_owned();
    if !branch_name.is_empty() {
        return Ok(branch_name);
    }

    let detached = Command::new("git")
        .arg("-C")
        .arg(root_path)
        .arg("rev-parse")
        .arg("--short")
        .arg("HEAD")
        .output()
        .map_err(|err| format!("git unavailable: {err}"))?;

    if detached.status.success() {
        let short = String::from_utf8_lossy(&detached.stdout).trim().to_owned();
        if !short.is_empty() {
            return Ok(format!("detached@{short}"));
        }
    }

    Ok("detached".to_owned())
}

fn count_porcelain_changes(output: &str) -> (usize, usize, usize) {
    let mut staged = 0usize;
    let mut unstaged = 0usize;
    let mut untracked = 0usize;

    for line in output.lines() {
        let mut chars = line.chars();
        let first = chars.next().unwrap_or(' ');
        let second = chars.next().unwrap_or(' ');

        if first == '?' && second == '?' {
            untracked += 1;
            continue;
        }
        if first != ' ' {
            staged += 1;
        }
        if second != ' ' {
            unstaged += 1;
        }
    }

    (staged, unstaged, untracked)
}

#[cfg(test)]
mod tests {
    use super::{count_porcelain_changes, resolve_git_branch_name};
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn git_available() -> bool {
        Command::new("git").arg("--version").output().is_ok()
    }

    fn remove_dir_if_exists(path: &Path) {
        if path.exists() {
            let _ = fs::remove_dir_all(path);
        }
    }

    #[test]
    fn porcelain_counts_parse_correctly() {
        let output = " M a.txt\nM  b.txt\nMM c.txt\nA  d.txt\n?? e.txt\n";
        let (staged, unstaged, untracked) = count_porcelain_changes(output);
        assert_eq!(staged, 3);
        assert_eq!(unstaged, 2);
        assert_eq!(untracked, 1);
    }

    #[test]
    fn branch_resolution_handles_unborn_repository() {
        if !git_available() {
            return;
        }

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default();
        let temp_dir = std::env::temp_dir().join(format!("myterminal-c-sidebar-{nonce}"));
        remove_dir_if_exists(&temp_dir);
        fs::create_dir_all(&temp_dir).expect("temp repo dir should be created");

        let init = Command::new("git")
            .arg("-C")
            .arg(&temp_dir)
            .arg("init")
            .output()
            .expect("git init should run");
        assert!(
            init.status.success(),
            "git init failed: {}",
            String::from_utf8_lossy(&init.stderr)
        );

        let branch = resolve_git_branch_name(&temp_dir)
            .expect("branch name should be resolved in unborn repository");
        assert!(!branch.trim().is_empty());

        remove_dir_if_exists(&temp_dir);
    }
}
